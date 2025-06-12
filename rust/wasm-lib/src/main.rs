#![deny(unused_must_use)]

mod level;
mod object;
mod render;
mod state;
// mod text;
mod util;
mod utilgen;

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

struct App {
    window: Option<Arc<Window>>,
    state: Option<State>,

    // delta: f64,
    prev_time: i64,
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

        let mut state = State::new(canvas, size);

        // state.set_event_start(1732996643000.0);
        // state.set_now(0.0);
        // state.set_quality(1.0);
        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    // self.delta += 1 / 60;

                    let local_now: DateTime<Local> = Local::now();
                    let now = local_now.timestamp_millis();

                    // dbg!(local.to_string());

                    // let start = SystemTime::now();
                    // let now = start
                    //     .duration_since(UNIX_EPOCH)
                    //     .expect("Time went backwards")
                    //     .as_millis();

                    state.set_now(local_now);
                    state.render((now - self.prev_time) as f32);

                    self.prev_time = now;
                }

                self.window.as_ref().unwrap().request_redraw();
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

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App {
        window: None,
        state: None,
        // delta: 0.0,
        prev_time: 0,
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
