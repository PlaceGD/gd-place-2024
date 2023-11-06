use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct TextDraw {
    #[wasm_bindgen(skip)]
    pub text: String,
    pub size: f32,
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl TextDraw {
    #[wasm_bindgen]
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}
