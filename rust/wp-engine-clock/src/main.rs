#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(unused_must_use)]

mod config;
mod error;
mod level;
mod object;
mod render;
mod state;
mod util;
mod utilgen;

use std::backtrace::Backtrace;
use std::cell::RefCell;
use std::fs::File;
use std::num::NonZeroIsize;
use std::os::windows::raw::HANDLE;
use std::panic;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Local};

use simplelog::{CombinedLogger, LevelFilter, WriteLogger};

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{GetStockObject, BLACK_BRUSH, HBRUSH};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

use wgpu::rwh::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle,
};

use windows::Win32::System::Threading::{
    CreateWaitableTimerW, SetWaitableTimer, WaitForSingleObject, INFINITE,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetWindowLongPtrW,
    PeekMessageW, PostQuitMessage, RegisterClassW, SetWindowLongPtrW, ShowWindow, TranslateMessage,
    CW_USEDEFAULT, GWLP_USERDATA, MSG, PM_REMOVE, SW_SHOW, WM_CLOSE, WM_DESTROY, WM_QUIT, WM_SIZE,
    WNDCLASSW, WS_CHILD, WS_EX_NOREDIRECTIONBITMAP, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

use crate::config::Config;
use crate::error::AppError;
use crate::state::PendingState;
use crate::util::{to_wstr, HIWORD, LOWORD};

struct CustomWindow {
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
}

unsafe impl Sync for CustomWindow {}
unsafe impl Send for CustomWindow {}

impl HasDisplayHandle for CustomWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe { Ok(DisplayHandle::borrow_raw(self.display_handle)) }
    }
}

impl HasWindowHandle for CustomWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unsafe { Ok(WindowHandle::borrow_raw(self.window_handle)) }
    }
}

fn get_parent_hwnd_from_args() -> Option<HWND> {
    let args: Vec<String> = std::env::args().collect();
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        if arg == "-parentHWND" {
            if let Some(hwnd_str) = iter.next() {
                if let Ok(hwnd_val) = hwnd_str.parse::<isize>() {
                    return Some(HWND(hwnd_val as _));
                }
            }
        }
    }

    None
}

fn init_logging() {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        simplelog::Config::default(),
        File::create("log.log").unwrap(),
    )])
    .unwrap();
}

unsafe fn get_state(hwnd: HWND) -> Option<&'static RefCell<PendingState>> {
    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const RefCell<PendingState>;

    if ptr.is_null() {
        None
    } else {
        Some(&*ptr)
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_CLOSE => {
                DestroyWindow(hwnd).unwrap();
                LRESULT(0)
            }
            WM_DESTROY => {
                let state = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut RefCell<PendingState>;

                if !state.is_null() {
                    if let Some(state) = (&*state).borrow_mut().ready() {
                        state.render.instance.poll_all(true);
                    }

                    drop(Box::from_raw(state));
                }

                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_SIZE => {
                let width = LOWORD(lparam.0 as u32) as u32;
                let height = HIWORD(lparam.0 as u32) as u32;

                if let Some(pending_state) = get_state(hwnd) {
                    if let Some(state) = pending_state.borrow_mut().ready() {
                        state.resize(width, height);
                    }
                }

                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn run_event_loop(
    target_frame_time: f32,
    mut render: impl FnMut(f32) -> Result<(), AppError>,
) -> Result<(), AppError> {
    unsafe {
        let mut msg = MSG::default();

        // Timing setup
        let mut frequency = 0i64;
        QueryPerformanceFrequency(&mut frequency).map_err(AppError::WindowsError)?;

        let mut last = 0i64;
        QueryPerformanceCounter(&mut last).map_err(AppError::WindowsError)?;

        'main_loop: loop {
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
                if msg.message == WM_QUIT {
                    break 'main_loop Ok(());
                }

                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let mut now = 0i64;
            QueryPerformanceCounter(&mut now).map_err(AppError::WindowsError)?;
            let delta = (now - last) as f64 / frequency as f64;
            last = now;

            render(delta as f32)?;

            let sleep_duration = (target_frame_time - delta as f32).max(0.0);
            if sleep_duration > 0.0 {
                let timer =
                    CreateWaitableTimerW(None, true, None).map_err(AppError::WindowsError)?;

                let due_time = -((Duration::from_secs_f32(sleep_duration).as_nanos() as i64) / 100);
                SetWaitableTimer(timer, &due_time, 0, None, None, false).unwrap();
                WaitForSingleObject(timer, INFINITE);
            }
        }
    }
}

unsafe fn start_app(config: Config) -> Result<(), AppError> {
    let parent_hwnd = get_parent_hwnd_from_args();

    let hinstance = GetModuleHandleW(None).map_err(AppError::WindowsError)?;
    let class_name = to_wstr("GD Place Countdown Clock Window Class");

    let wc = WNDCLASSW {
        hInstance: hinstance.into(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        lpfnWndProc: Some(window_proc),
        hbrBackground: HBRUSH(GetStockObject(BLACK_BRUSH).0),
        ..Default::default()
    };

    RegisterClassW(&wc);

    let hwnd = CreateWindowExW(
        WS_EX_NOREDIRECTIONBITMAP,
        PCWSTR(class_name.as_ptr()),
        PCWSTR(to_wstr("GD Place Countdown Clock").as_ptr()),
        WS_VISIBLE
            | (if parent_hwnd.is_some() {
                WS_CHILD
            } else {
                WS_OVERLAPPEDWINDOW
            }),
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        800,
        600,
        parent_hwnd,
        None,
        Some(hinstance.into()),
        None,
    )
    .map_err(AppError::WindowsError)?;

    ShowWindow(hwnd.into(), SW_SHOW)
        .ok()
        .map_err(AppError::WindowsError)?;

    let mut window_handle = Win32WindowHandle::new(NonZeroIsize::new_unchecked(hwnd.0 as isize));
    window_handle.hinstance = Some(NonZeroIsize::new_unchecked(hinstance.0 as isize));

    let display_handle = WindowsDisplayHandle::new();

    let window = Arc::new(CustomWindow {
        window_handle: RawWindowHandle::Win32(window_handle),
        display_handle: RawDisplayHandle::Windows(display_handle),
    });

    let target_frame_time = 1.0 / config.general.fps_cap as f32;

    let mut state = PendingState::new();
    state.init_state(window, (800, 600), config)?;

    let boxed_state = Box::new(RefCell::new(state));
    SetWindowLongPtrW(hwnd, GWLP_USERDATA, Box::into_raw(boxed_state) as isize);

    run_event_loop(target_frame_time, move |delta_time| {
        if let Some(cell) = unsafe { get_state(hwnd) } {
            if let Some(state) = cell.borrow_mut().ready() {
                let local_now: DateTime<Local> = Local::now();
                state.set_now(local_now);
                state.render(delta_time)?;
            }
        }
        Ok(())
    })?;

    Ok(())
}

fn main() -> Result<(), AppError> {
    panic::set_hook(Box::new(|info| {
        let message = if let Some(string) = info.payload().downcast_ref::<String>() {
            string.to_owned()
        } else if let Some(string) = info.payload().downcast_ref::<&'static str>() {
            string.to_string()
        } else {
            format!("{:?}", info.payload())
        };

        let bt = Backtrace::force_capture();

        log::error!("[CLOCK] fatal error occured: {message}\n[backtrace]:\n{bt}");
    }));

    let config = Config::from_file_or_default()?;

    if config.general.logging {
        init_logging();
    }

    log::info!("[CLOCK] Starting app");

    unsafe {
        match start_app(config) {
            Ok(..) => (),
            Err(e) => {
                log::error!("[CLOCK] failed to start app: {e}");
                // write_error_log(format!("{e}"), None);
            }
        };
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct RustError {
    typ: ErrorType,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorType {
    InvalidObjectId(u16),
    InvalidObjectString(&'static str),
    ObjectSerialization,
    ObjectDeserialization,
}

impl From<ErrorType> for RustError {
    fn from(value: ErrorType) -> Self {
        RustError { typ: value }
    }
}
impl<T> From<ErrorType> for Result<T, RustError> {
    fn from(value: ErrorType) -> Self {
        Err(value.into())
    }
}

impl RustError {
    pub fn display(&self) -> String {
        match self.typ {
            ErrorType::InvalidObjectId(id) => format!("Invalid object ID: {id}."),
            ErrorType::InvalidObjectString(e) => {
                format!("Invalid object string deserialization. ({e})")
            }
            ErrorType::ObjectSerialization => {
                "Failed to serialize object. Please report this issue.".to_string()
            }
            ErrorType::ObjectDeserialization => {
                "Failed to deserialize object. Please report this issue.".to_string()
            }
        }
    }
}
