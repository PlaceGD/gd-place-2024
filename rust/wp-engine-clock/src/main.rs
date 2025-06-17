#![deny(unused_must_use)]

mod level;
mod object;
mod render;
mod state;
// mod text;
mod config;
mod error;
mod util;
mod utilgen;

use std::backtrace::Backtrace;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::{fs, panic};

use chrono::{DateTime, Local};
use render::state::RenderState;

use state::State;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::monitor::Fullscreen;
use winit::window::{Window, WindowAttributes, WindowButtons, WindowId};

use crate::config::Config;
use crate::error::AppError;
use crate::state::PendingState;

// pub async fn create_view(
//     canvas: OffscreenCanvas,
//     spritesheet_data: &[u8],
//     spritesheet_width: u32,
//     spritesheet_height: u32,
// ) -> Result<State, StateError> {
// Ok(State::new(
//     RenderState::new_canvas(
//         canvas,
//         spritesheet_data,
//         spritesheet_width,
//         spritesheet_height,
//     )
//     .await?,
// ))

//     // StateWrapper::new(desen::new_app_canvas(canvas, |app| {
//     //     State::init(app, spritesheet_data)
//     // }))
// }

struct App {
    window: Option<Arc<Box<dyn Window + 'static>>>,
    state: PendingState,
    config: Option<Config>,

    last_frame_instant: Instant,
    next_frame_time: Instant,

    frame_duration: Duration,
}

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let mut window_attributes = WindowAttributes::default()
            .with_resizable(true)
            .with_visible(false)
            .with_title("GD Place Countdown Clock")
            .with_transparent(true);

        #[cfg(target_os = "windows")]
        {
            // enable transparency support with DirectComposition, which is what the fork of
            // WGPU enables in DX12
            window_attributes = window_attributes.with_platform_attributes(Box::new(
                winit::platform::windows::WindowAttributesWindows::default()
                    .with_no_redirection_bitmap(true),
            ));
        }

        #[cfg(not(debug_assertions))]
        {
            window_attributes = window_attributes
                .with_enabled_buttons(WindowButtons::empty())
                .with_resizable(false)
                .with_decorations(false)
                .with_fullscreen(Some(Fullscreen::Borderless(None)));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let size = window.surface_size();

        self.window = Some(Arc::clone(&window));

        let config = self.config.take().ok_or(AppError::ConfigTaken).unwrap();

        self.state.init_state(window, size, config).unwrap();

        let now = Instant::now();
        self.last_frame_instant = now;
        self.next_frame_time = now + self.frame_duration;
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame_time));
    }

    fn new_events(&mut self, event_loop: &dyn ActiveEventLoop, _: winit::event::StartCause) {
        if Instant::now() >= self.next_frame_time {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame_time));
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = self.state.ready() {
                    let now_instant = Instant::now();
                    let delta_time = now_instant
                        .duration_since(self.last_frame_instant)
                        .as_secs_f32();

                    let local_now: DateTime<Local> = Local::now();
                    state.set_now(local_now);
                    state.render(delta_time).unwrap();

                    self.last_frame_instant = now_instant;
                    self.next_frame_time = now_instant + self.frame_duration;

                    event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame_time));
                }
            }
            WindowEvent::SurfaceResized(size) => {
                if let Some(state) = self.state.ready() {
                    state.resize(size.width, size.height);
                }
            }
            _ => (),
        }
    }
}

fn start_app() -> Result<(), AppError> {
    let config = Config::from_file_or_default()?;

    let event_loop = EventLoop::new().map_err(AppError::EventLoopError)?;

    let now = Instant::now();
    let mut app = App {
        window: None,
        state: PendingState::new(),
        last_frame_instant: now,
        next_frame_time: now,

        frame_duration: Duration::from_secs_f64(1.0 / config.general.fps_cap as f64),
        config: Some(config),
    };

    event_loop
        .run_app(&mut app)
        .map_err(AppError::EventLoopError)?;

    Ok(())
}

fn write_error_log(message: String, bt: Option<Backtrace>) {
    eprintln!("ERROR:\n{message}");

    let bt_string = bt.map_or_else(|| String::new(), |bt| format!("\n{bt}"));

    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("./error.log")
    {
        Ok(mut file) => {
            let _ = write!(file, "[ERROR]:\n{message}{bt_string}\n\n");
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}

fn main() -> Result<(), AppError> {
    let _ = OpenOptions::new().truncate(true).open("./error.log");

    panic::set_hook(Box::new(|info| {
        let message = if let Some(string) = info.payload().downcast_ref::<String>() {
            string.to_owned()
        } else if let Some(string) = info.payload().downcast_ref::<&'static str>() {
            string.to_string()
        } else {
            format!("{:?}", info.payload())
        };

        let bt = Backtrace::force_capture();

        write_error_log(message, Some(bt));
    }));

    match start_app() {
        Ok(..) => (),
        Err(e) => {
            write_error_log(format!("{e}"), None);
        }
    };

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
