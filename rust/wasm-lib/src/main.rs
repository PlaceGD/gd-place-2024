#![deny(unused_must_use)]

mod level;
mod object;
mod render;
mod state;
// mod text;
mod util;
mod utilgen;

use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};
use render::state::{RenderState, StateError};

use state::State;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowButtons, WindowId};

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

// TODO: setting: change fps
const TARGET_FPS: f64 = 60.0;

struct App {
    window: Option<Arc<Window>>,
    state: Option<State>,

    last_frame_instant: Instant,
    next_frame_time: Instant,

    frame_duration: Duration,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_resizable(true)
            // .with_enabled_buttons(WindowButtons::empty())
            .with_title("GD Place Countdown Clock")
            .with_transparent(true);
        // .with_decorations(false);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let size = window.outer_size();

        self.window = Some(Arc::clone(&window));

        let canvas =
            futures::executor::block_on(RenderState::new_canvas(Arc::clone(&window), size))
                .expect("TODO: handle me");

        let state = State::new(canvas, size);

        self.state = Some(state);

        let now = Instant::now();
        self.last_frame_instant = now;
        self.next_frame_time = now + self.frame_duration;
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame_time));
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        if Instant::now() >= self.next_frame_time {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame_time));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    let now_instant = Instant::now();
                    let delta_time = now_instant
                        .duration_since(self.last_frame_instant)
                        .as_secs_f32();

                    let local_now: DateTime<Local> = Local::now();
                    state.set_now(local_now);
                    state.render(delta_time);

                    self.last_frame_instant = now_instant;
                    self.next_frame_time = now_instant + self.frame_duration;

                    event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame_time));
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(state) = &mut self.state {
                    state.resize(size.width, size.height);
                }
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), StateError> {
    let event_loop = EventLoop::new().unwrap();

    let now = Instant::now();
    let mut app = App {
        window: None,
        state: None,
        last_frame_instant: now,
        next_frame_time: now,

        frame_duration: Duration::from_secs_f64(1.0 / TARGET_FPS),
    };
    event_loop.run_app(&mut app).unwrap();

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
