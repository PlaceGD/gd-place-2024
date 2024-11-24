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

use winit::window::Window;
pub use {object::GDObjectOpt, state::State};

pub async fn create_view(
    window: Arc<Window>,
    spritesheet_data: &[u8],
    spritesheet_width: u32,
    spritesheet_height: u32,
) -> Result<State, StateError> {
    Ok(State::new(
        RenderState::new_window(
            window,
            spritesheet_data,
            spritesheet_width,
            spritesheet_height,
        )
        .await?,
    ))

    // StateWrapper::new(desen::new_app_canvas(canvas, |app| {
    //     State::init(app, spritesheet_data)
    // }))
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
