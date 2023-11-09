use desen::frame::FrameTransformMatrix;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct TextDraw {
    #[wasm_bindgen(skip)]
    pub text: String,
    pub font_size: f32,
    #[wasm_bindgen(skip)]
    pub transform: FrameTransformMatrix,
    #[wasm_bindgen(skip)]
    pub extra_style: String,
}

#[wasm_bindgen]
impl TextDraw {
    #[wasm_bindgen]
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
    #[wasm_bindgen]
    pub fn get_extra_style(&self) -> String {
        self.extra_style.clone()
    }
    #[wasm_bindgen]
    pub fn get_css_transform(&self) -> String {
        format!(
            "matrix3d(
            {}, {}, 0, 0,
            {}, {}, 0, 0,
            0, 0, 1, 0,
            {}, {}, 0, 1
        )",
            self.transform.m11,
            -self.transform.m21,
            self.transform.m12,
            -self.transform.m22,
            self.transform.m13,
            -self.transform.m23
        )
    }
}
