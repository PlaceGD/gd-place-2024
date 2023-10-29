mod layer;
mod level;
mod object;
mod util;

use desen::{
    frame::BlendMode,
    state::{AppState, CanvasAppState, LoadedTexture},
    CanvasAppBundle,
};
use level::Level;
use nalgebra::{vector, Vector2, Vector4};
use object::GDObject;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const LEVEL_WIDTH_BLOCKS: u32 = 400;
const LEVEL_HEIGHT_BLOCKS: u32 = 80;
const LEVEL_WIDTH_UNITS: u32 = LEVEL_WIDTH_BLOCKS * 30;
const LEVEL_HEIGHT_UNITS: u32 = LEVEL_HEIGHT_BLOCKS * 30;

struct State {
    time: f32,
    width: u32,
    height: u32,

    background: LoadedTexture,
    camera_pos: Vector2<f32>,
    zoom: f32,

    level: Level,

    bg_color: (u8, u8, u8, u8),
}

impl State {
    pub fn get_zoom_scale(&self) -> f32 {
        2.0f32.powf(self.zoom / 12.0)
    }
}

impl AppState for State {
    fn update(&mut self, delta: f32) {
        self.time += delta;
    }

    fn view(&self, frame: &mut desen::frame::Frame) {
        let zoom_scale = self.get_zoom_scale();
        {
            frame.fill(
                self.bg_color.0,
                self.bg_color.1,
                self.bg_color.2,
                self.bg_color.3,
            );
            let dimension = self.width.max(self.height) as f32;
            frame.set_current_texture(self.background);
            frame.draw_image(
                -dimension / 2.0,
                -dimension / 2.0,
                Some(dimension),
                Some(dimension),
                true,
            );
        }

        // grid drawing
        // have to do some shit manually to make sure the lines are pixel aligned
        {
            frame.push();

            let tx = (-self.camera_pos.x * zoom_scale).floor();
            let ty = (-self.camera_pos.y * zoom_scale).floor();

            frame.translate(tx, ty);

            frame.stroke_weight(4.0);
            frame.stroke(0, 0, 0, 255);
            frame.no_fill();
            frame.rect(
                0.0,
                0.0,
                LEVEL_WIDTH_UNITS as f32 * zoom_scale,
                LEVEL_HEIGHT_UNITS as f32 * zoom_scale,
            );
            // frame.line(-2.0, 0.0, LEVEL_SIZE_UNITS.x as f32 * zoom_scale, 0.0);
            // frame.line(0.0, 0.0, 0.0, LEVEL_SIZE_UNITS.y as f32 * zoom_scale);
            // frame.line(-2.0, LEVEL_SIZE_UNITS.y as f32 * zoom_scale, LEVEL_SIZE_UNITS.x as f32 * zoom_scale, LEVEL_SIZE_UNITS.y as f32 * zoom_scale);
            // frame.line(0.0, 0.0, 0.0, LEVEL_SIZE_UNITS.y as f32 * zoom_scale);

            frame.stroke_weight(1.0);
            frame.stroke(0, 0, 0, map!(self.zoom, -24.0, 24.0, 0.0, 255.0) as u8);
            if self.zoom >= -24.0 {
                for x in 1..=LEVEL_WIDTH_BLOCKS {
                    frame.line(
                        (x as f32 * 30.0 * zoom_scale).floor() + 0.5,
                        0.0,
                        (x as f32 * 30.0 * zoom_scale).floor() + 0.5,
                        LEVEL_HEIGHT_UNITS as f32 * zoom_scale,
                    );
                }
                for y in 1..=LEVEL_HEIGHT_BLOCKS {
                    frame.line(
                        0.0,
                        (y as f32 * 30.0 * zoom_scale).floor() + 0.5,
                        LEVEL_WIDTH_UNITS as f32 * zoom_scale,
                        (y as f32 * 30.0 * zoom_scale).floor() + 0.5,
                    );
                }
            }
            frame.pop()
        }

        frame.scale(zoom_scale, zoom_scale);
        frame.translate(-self.camera_pos.x, -self.camera_pos.y);

        frame.no_stroke();
        frame.fill(255, 0, 0, 127);
        frame.set_blend_mode(BlendMode::AdditiveSquaredAlpha);
        frame.rect(0.0, 0.0, 100.0, 100.0);
        frame.rect(50.0, 50.0, 100.0, 100.0);
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
            bg_color: (40, 125, 255, 255),
            background: loader.load_texture_bytes(include_bytes!("../background.png")),
            level: Level::default(),
        }
    }
}

#[wasm_bindgen]
struct StateWrapper {
    bundle: CanvasAppBundle<State>,
    // canvas: web_sys::HtmlCanvasElement,
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

    pub fn get_camera_pos(&self) -> Vec<f32> {
        vec![
            self.bundle.state.camera_pos.x,
            self.bundle.state.camera_pos.y,
        ]
    }
    pub fn set_camera_pos(&mut self, x: f32, y: f32) {
        self.bundle.state.camera_pos = vector![
            x.clamp(0.0, LEVEL_WIDTH_UNITS as f32),
            y.clamp(0.0, LEVEL_HEIGHT_UNITS as f32)
        ];
    }
    pub fn set_bg_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.bundle.state.bg_color = (r, g, b, a);
    }
    pub fn get_bg_color(&mut self) -> Vec<u8> {
        let (r, g, b, a) = self.bundle.state.bg_color;
        vec![r, g, b, a]
    }

    pub fn get_zoom(&self) -> f32 {
        self.bundle.state.zoom
    }
    pub fn set_zoom(&mut self, v: f32) {
        self.bundle.state.zoom = v
    }
    pub fn get_zoom_scale(&self) -> f32 {
        self.bundle.state.get_zoom_scale()
    }

    pub fn get_world_pos(&self, x: f32, y: f32) -> Vec<f32> {
        let (cx, cy) = (
            self.bundle.state.camera_pos.x,
            self.bundle.state.camera_pos.y,
        );
        let s = self.bundle.state.get_zoom_scale();
        vec![(x + s * cx) / s, (y + s * cy) / s]
    }

    pub fn add_object(&mut self, obj: GDObject) {}
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn create_view(canvas: web_sys::HtmlCanvasElement) -> StateWrapper {
    StateWrapper {
        bundle: desen::new_app_canvas::<State>(canvas),
        // canvas,
    }
    // State::new(canvas).await
}

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
