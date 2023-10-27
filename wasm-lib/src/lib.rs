use desen::{
    state::{AppState, CanvasAppState, LoadedTexture},
    CanvasAppBundle,
};
use nalgebra::{vector, Vector2};
use wasm_bindgen::prelude::*;

struct State {
    time: f32,
    width: u32,
    height: u32,

    background: LoadedTexture,
    camera_pos: Vector2<f32>,
    zoom: f32,
}

impl AppState for State {
    fn update(&mut self, delta: f32) {
        self.time += delta;
    }

    fn view(&self, frame: &mut desen::frame::Frame) {
        {
            frame.fill(40, 125, 255, 255);
            let dimension = self.width.max(self.height) as f32;
            frame.draw_image(
                self.background,
                -dimension / 2.0,
                -dimension / 2.0,
                Some(dimension),
                Some(dimension),
                true,
            );
        }

        frame.translate(-self.camera_pos.x, -self.camera_pos.y);

        const LEVEL_SIZE_BLOCKS: Vector2<u32> = vector![80, 80];
        const LEVEL_SIZE_UNITS: Vector2<u32> = vector![80 * 30, 80 * 30];

        frame.stroke_weight(2.0);
        frame.stroke(0, 0, 0, 255);
        frame.line(0.0, 0.0, LEVEL_SIZE_UNITS.x as f32, 0.0);
        frame.line(0.0, 0.0, 0.0, LEVEL_SIZE_UNITS.y as f32);

        frame.stroke_weight(1.0);
        for x in 0..=LEVEL_SIZE_BLOCKS.x {
            frame.line(
                x as f32 * 30.0 + 0.5,
                0.0,
                x as f32 * 30.0 + 0.5,
                LEVEL_SIZE_UNITS.y as f32,
            );
        }
        for y in 0..=LEVEL_SIZE_BLOCKS.y {
            frame.stroke(0, 0, 0, 255);
            frame.line(
                0.0,
                y as f32 * 30.0 + 0.5,
                LEVEL_SIZE_UNITS.x as f32,
                y as f32 * 30.0 + 0.5,
            );
        }
    }
}

impl CanvasAppState for State {
    fn init(loader: &mut desen::state::ResourceLoader) -> Self {
        // need this so that the atlas is nonzero lol!!!
        // technically not rly cause we are importing other stuff
        // but just in case
        loader.load_texture_bytes(include_bytes!("../biddledoo.png"));
        Self {
            time: 0.0,
            width: 10,
            height: 10,
            camera_pos: vector![0.0, 0.0],
            zoom: 0.0,
            background: loader.load_texture_bytes(include_bytes!("../background.png")),
        }
    }
}

#[wasm_bindgen]
struct StateWrapper {
    bundle: CanvasAppBundle<State>,
}

#[wasm_bindgen]
impl StateWrapper {
    pub fn pub_render(&mut self, delta: f32) {
        self.bundle.render(delta);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.bundle.resize(width, height);
        (self.bundle.state.width, self.bundle.state.height) = self.bundle.get_size();
    }

    pub fn get_camera_x(&self) -> f32 {
        self.bundle.state.camera_pos.x
    }
    pub fn get_camera_y(&self) -> f32 {
        self.bundle.state.camera_pos.y
    }
    pub fn set_camera_pos(&mut self, x: f32, y: f32) {
        self.bundle.state.camera_pos = vector![x, y];
    }
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn create_view(canvas: web_sys::HtmlCanvasElement) -> StateWrapper {
    StateWrapper {
        bundle: desen::new_app_canvas::<State>(canvas),
    }
    // State::new(canvas).await
}

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
