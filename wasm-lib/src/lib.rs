#![deny(unused_must_use)]

mod layer;
mod level;
mod object;
mod state;
mod text;
mod util;
mod utilgen;

// use colored::control::set_override;
use wasm_bindgen::prelude::*;

// pub fn create_view_aux(canvas: web_sys::HtmlCanvasElement) -> crate::state::StateWrapper {}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn create_view(
    canvas: web_sys::HtmlCanvasElement,
    spritesheet_data: &[u8],
) -> crate::state::StateWrapper {
    crate::state::StateWrapper::new(desen::new_app_canvas(canvas, |app| {
        crate::state::State::init(app, spritesheet_data)
    }))
}

#[wasm_bindgen]
pub fn funtest() -> String {
    unsafe { String::from_utf8_unchecked(vec![97, 98, 99, 200]) }
}

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //set_override(true);
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
