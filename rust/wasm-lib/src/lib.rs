#![deny(unused_must_use)]

mod level;
mod object;
mod render;
mod state;
// mod text;
mod util;
mod utilgen;

use render::state::{RenderState, StateError};
use wasm_bindgen::prelude::*;

use state::State;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub async fn create_view(
    canvas: HtmlCanvasElement,
    spritesheet_data: &[u8],
    spritesheet_width: u32,
    spritesheet_height: u32,
) -> Result<State, StateError> {
    Ok(State::new(
        RenderState::new_canvas(
            canvas,
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

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
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

#[wasm_bindgen]
impl RustError {
    #[wasm_bindgen]
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
