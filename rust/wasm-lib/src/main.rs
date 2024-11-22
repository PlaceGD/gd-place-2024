#![deny(unused_must_use)]

mod level;
mod object;
mod render;
mod state;
// mod text;
mod util;
mod utilgen;

use std::sync::Arc;

use render::state::{RenderState, StateError};

use state::State;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

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
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        self.window = Some(Arc::clone(&window));

        let canvas = futures::executor::block_on(RenderState::new_canvas(Arc::clone(&window)))
            .expect("TODO: handle me");

        self.state = Some(State::new(canvas));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                // println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &mut self.state {
                    state.render(0.0)
                }

                self.window.as_ref().unwrap().request_redraw();
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
