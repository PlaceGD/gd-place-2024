#![deny(unused_must_use)]

mod layer;
mod level;
mod object;
mod state;
mod util;
mod utilgen;

use colored::control::set_override;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn create_view(canvas: web_sys::HtmlCanvasElement) -> crate::state::StateWrapper {
    crate::state::StateWrapper::new(desen::new_app_canvas::<crate::state::State>(canvas))
    // State::new(canvas).await
}

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    set_override(true);
}

#[wasm_bindgen]
pub struct RustError {
    typ: ErrorType,
}

pub enum ErrorType {
    InvalidObjectId(u16),
    InvalidObjectString,
    ObjectSerialization,
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
            ErrorType::InvalidObjectString => "Invalid object string deserialization.".to_string(),
            ErrorType::ObjectSerialization => {
                "Failed to serialize object. Please report this issue.".to_string()
            }
        }
    }
}
