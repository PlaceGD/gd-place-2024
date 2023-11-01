mod layer;
mod level;
mod object;
mod state;
mod util;
mod utilgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn create_view(canvas: web_sys::HtmlCanvasElement) -> crate::state::StateWrapper {
    crate::state::StateWrapper::new(desen::new_app_canvas::<crate::state::State>(canvas))
    // State::new(canvas).await
}

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
