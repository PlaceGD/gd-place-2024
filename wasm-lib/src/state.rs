use std::{collections::HashSet, f32::consts::PI};

use colored::Colorize;
use desen::{
    frame::{BlendMode, Frame, FrameTransform, FrameTransformMatrix},
    state::{AppState, CanvasAppState, LoadedTexture},
    CanvasAppBundle,
};
use the_nexus::packing::SpriteInfo;
use wasm_bindgen::prelude::*;

use crate::{
    layer::{ZLayer, Z_LAYERS},
    level::{
        ChunkCoord, ChunkInfo, DbKey, Level, CHUNK_SIZE_BLOCKS, CHUNK_SIZE_UNITS,
        LEVEL_HEIGHT_BLOCKS, LEVEL_HEIGHT_UNITS, LEVEL_RECT_BLOCKS, LEVEL_WIDTH_BLOCKS,
        LEVEL_WIDTH_UNITS,
    },
    log, map,
    object::{GDColor, GDObject},
    text::TextDraw,
    util::{get_chunk_coord, get_max_bounding_box, now, point_in_triangle, Rect},
    utilgen::{get_detail_sprite, get_main_sprite, get_object_info},
    ErrorType, RustError,
};

use nalgebra::{vector, Vector2, Vector4};
pub struct State {
    time: f32,

    width: u32,
    height: u32,

    background: LoadedTexture,
    spritesheet: LoadedTexture,

    camera_pos: Vector2<f32>,
    zoom: f32,

    level: Level,

    bg_color: (u8, u8, u8, u8),

    preview_object: GDObject,
    show_preview: bool,

    selected_object: Option<DbKey>,
    select_depth: u32,

    text_draws: Vec<TextDraw>,
}

impl State {
    pub fn get_zoom_scale(&self) -> f32 {
        2.0f32.powf(self.zoom / 12.0)
    }
    pub fn view_transform(&self) -> FrameTransformMatrix {
        let scale = self.get_zoom_scale();
        FrameTransform::series_mat([
            FrameTransform::Scale { x: scale, y: scale },
            FrameTransform::Translate {
                x: -self.camera_pos.x,
                y: -self.camera_pos.y,
            },
        ])
    }
}

fn obj_transform(obj: &GDObject) -> FrameTransformMatrix {
    FrameTransform::series_mat([
        FrameTransform::Translate { x: obj.x, y: obj.y },
        FrameTransform::Rotate(-obj.rotation as f32 * PI / 180.0),
        FrameTransform::Scale {
            x: if obj.flip_x { -0.25 } else { 0.25 } * obj.scale,
            y: if obj.flip_y { -0.25 } else { 0.25 } * obj.scale,
        },
    ])
}
fn padded_obj_rect(obj: &GDObject, pad: f32) -> Rect<f32> {
    let mut rect_size = get_max_bounding_box(obj.id as u32).unwrap_or((10.0, 10.0));

    rect_size.0 += pad;
    rect_size.1 += pad;

    Rect::new(
        -rect_size.0 / 2.0,
        -rect_size.1 / 2.0,
        rect_size.0,
        rect_size.1,
    )
}

impl AppState for State {
    fn update(&mut self, delta: f32) {
        self.time += delta;
        // self.preview_object.rotation += 1;
        // let viewable = self.get_viewable_chunks();

        // for w in (0..viewable.len()).step_by(2) {
        //     let x = viewable[w];
        //     let y = viewable[w + 1];
        //     frame.stroke(0, 255, 0, 255);
        //     frame.stroke_weight(4.0);
        //     frame.no_fill();
        //     frame.rect(
        //         x as f32 * 20.0 * 30.0,
        //         y as f32 * 20.0 * 30.0,
        //         20.0 * 30.0,
        //         20.0 * 30.0,
        //     );
        // }
    }

    fn view(&mut self, frame: &mut desen::frame::Frame) {
        self.draw_text(frame, "bibby", 0.0, 0.0, 10.0);
        self.draw_text(frame, "Gaga", 100.0, 20.0, 20.0);

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

        frame.set_current_texture(self.spritesheet);
        frame.transform(FrameTransform::Custom(self.view_transform()));

        {
            let mut view_rect = self.get_camera_world_rect();
            view_rect.w /= 2.0;
            view_rect.h /= 2.0;
            view_rect.x += view_rect.w / 2.0;
            view_rect.y += view_rect.h / 2.0;

            frame.fill(255, 0, 0, 127);
            frame.no_stroke();
            for (x, y) in view_rect.corners() {
                frame.circle(x, y, 10.0);
            }
        }

        frame.stroke_weight(4.0);
        frame.no_fill();

        let now = now();

        for (&ChunkCoord { x, y }, chunk) in &self.level.chunks {
            frame.stroke(
                0,
                255,
                0,
                map!(
                    now - chunk.last_time_visible,
                    0.0,
                    UNLOAD_CHUNK_TIME * 1000.0,
                    255.0,
                    0.0
                ) as u8,
            );
            frame.rect(
                x as f32 * 20.0 * 30.0,
                y as f32 * 20.0 * 30.0,
                20.0 * 30.0,
                20.0 * 30.0,
            );
        }

        {
            let draw_obj_sprite = |frame: &mut Frame, sprite: &SpriteInfo, obj: &GDObject| {
                frame.push();
                frame.transform(FrameTransform::Custom(obj_transform(obj)));
                frame.draw_image_cropped(
                    -(sprite.size.0 as f32) / 2.0 + sprite.offset.0,
                    -(sprite.size.1 as f32) / 2.0 - sprite.offset.1,
                    None,
                    None,
                    (
                        sprite.pos.0 as f32,
                        sprite.pos.1 as f32,
                        sprite.size.0 as f32,
                        sprite.size.1 as f32,
                    ),
                    true,
                );
                frame.pop()
            };
            let draw_preview_sprite = |frame: &mut Frame,
                                       layer: ZLayer,
                                       z_order: i8,
                                       draw_detail: bool,
                                       if_blending: bool| {
                let (color, sprite_fn): (GDColor, fn(u32) -> Option<SpriteInfo>) = if draw_detail {
                    (self.preview_object.detail_color, get_detail_sprite)
                } else {
                    (self.preview_object.main_color, get_main_sprite)
                };

                if self.show_preview
                    && self.preview_object.z_layer == layer
                    && self.preview_object.z_order == z_order
                    && color.blending == if_blending
                {
                    if let Some(sprite) = sprite_fn(self.preview_object.id as u32) {
                        frame.fill(color.r, color.g, color.b, color.opacity);
                        draw_obj_sprite(frame, &sprite, &self.preview_object);
                    }
                }
            };
            let set_fill_if_selected = |frame: &mut Frame, key: DbKey, lighter: bool| {
                if self.selected_object == Some(key) {
                    let c = map!(
                        (self.time / 3.0).sin(),
                        -1.0,
                        1.0,
                        if lighter { 150.0 } else { 100.0 },
                        if lighter { 255.0 } else { 200.0 }
                    ) as u8;

                    frame.fill(255, c, c, 255);
                }
            };

            for layer in Z_LAYERS {
                frame.set_blend_mode(BlendMode::AdditiveSquaredAlpha);
                for z_order in -50..=50 {
                    self.level.foreach_obj_in_z(*layer, z_order, |key, obj| {
                        if let Some(sprite) = get_main_sprite(obj.id as u32) {
                            if obj.main_color.blending {
                                frame.fill(
                                    obj.main_color.r,
                                    obj.main_color.g,
                                    obj.main_color.b,
                                    obj.main_color.opacity,
                                );
                                set_fill_if_selected(frame, key, false);
                                draw_obj_sprite(frame, &sprite, obj);
                            }
                        }
                    });
                    draw_preview_sprite(frame, *layer, z_order, false, true);
                }
                frame.set_blend_mode(BlendMode::Normal);
                for z_order in -50..=50 {
                    self.level.foreach_obj_in_z(*layer, z_order, |key, obj| {
                        if let Some(sprite) = get_main_sprite(obj.id as u32) {
                            if !obj.main_color.blending {
                                frame.fill(
                                    obj.main_color.r,
                                    obj.main_color.g,
                                    obj.main_color.b,
                                    obj.main_color.opacity,
                                );
                                set_fill_if_selected(frame, key, false);
                                draw_obj_sprite(frame, &sprite, obj);
                            }
                        }
                        if let Some(sprite) = get_detail_sprite(obj.id as u32) {
                            if !obj.detail_color.blending {
                                frame.fill(
                                    obj.detail_color.r,
                                    obj.detail_color.g,
                                    obj.detail_color.b,
                                    obj.detail_color.opacity,
                                );
                                set_fill_if_selected(frame, key, true);
                                draw_obj_sprite(frame, &sprite, obj);
                            }
                        }
                    });
                    draw_preview_sprite(frame, *layer, z_order, false, false);
                    draw_preview_sprite(frame, *layer, z_order, true, false);
                }
                frame.set_blend_mode(BlendMode::AdditiveSquaredAlpha);
                for z_order in -50..=50 {
                    self.level.foreach_obj_in_z(*layer, z_order, |key, obj| {
                        if let Some(sprite) = get_detail_sprite(obj.id as u32) {
                            if obj.detail_color.blending {
                                frame.fill(
                                    obj.detail_color.r,
                                    obj.detail_color.g,
                                    obj.detail_color.b,
                                    obj.detail_color.opacity,
                                );
                                set_fill_if_selected(frame, key, true);
                                draw_obj_sprite(frame, &sprite, obj);
                            }
                        }
                    });
                    draw_preview_sprite(frame, *layer, z_order, true, true);
                }
            }
            frame.set_blend_mode(BlendMode::Normal);

            let highlight_obj = if self.show_preview {
                Some((&self.preview_object, (100, 255, 100)))
            } else if let Some(d) = self.selected_object {
                self.level.get_obj_by_key(d).map(|v| (v, (255, 100, 100)))
            } else {
                None
            };

            if let Some((obj, color)) = highlight_obj {
                frame.push();
                frame.transform(FrameTransform::Custom(obj_transform(obj)));

                let rect = padded_obj_rect(obj, 30.0);

                frame.no_fill();

                const IDEAL_DASH_LEN: f32 = 30.0;
                let dash_len =
                    rect.perimeter() / (rect.perimeter() / (IDEAL_DASH_LEN * 2.0)).round() / 2.0;

                let mut offset = self.time * 2.0;

                for ((x0, y0), (x1, y1)) in rect.sides() {
                    offset = self.dashed_line(frame, x0, y0, x1, y1, dash_len, offset, color, 8.0);
                }

                frame.pop();
            }
        }
    }
}

fn chunk_rect_blocks(x: i32, y: i32) -> Rect<i32> {
    Rect::new(
        x * CHUNK_SIZE_BLOCKS as i32,
        y * CHUNK_SIZE_BLOCKS as i32,
        CHUNK_SIZE_BLOCKS as i32,
        CHUNK_SIZE_BLOCKS as i32,
    )
}
fn chunk_rect_units(x: i32, y: i32) -> Rect<i32> {
    Rect::new(
        x * CHUNK_SIZE_UNITS as i32,
        y * CHUNK_SIZE_UNITS as i32,
        CHUNK_SIZE_UNITS as i32,
        CHUNK_SIZE_UNITS as i32,
    )
}

impl State {
    #[allow(clippy::too_many_arguments)]
    fn dashed_line(
        &self,
        frame: &mut Frame,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        dash_len: f32,
        offset: f32,
        (r, g, b): (u8, u8, u8),
        weight: f32,
    ) -> f32 {
        let start = vector![x0, y0];
        let end = vector![x1, y1];
        let to = end - start;
        let dir = to.normalize();

        macro_rules! line {
            ($a:expr, $l:expr) => {{
                let pb = start + dir * ($a + $l);
                let pa = start + dir * $a;
                frame.no_fill();
                frame.stroke(r, g, b, 255);
                frame.stroke_weight(weight);
                frame.line(pa.x, pa.y, pb.x, pb.y);

                frame.no_stroke();
                frame.fill(r, g, b, 255);
                frame.circle(pa.x, pa.y, weight / 2.0);
                frame.circle(pb.x, pb.y, weight / 2.0);
            }};
        }

        let offset = offset.rem_euclid(dash_len * 2.0);
        if offset > dash_len {
            let bit = offset - dash_len;

            // frame.stroke(255, 0, 0, 255);
            line!(0.0, bit);
        }
        let (full, rem) = {
            let d = (to.magnitude() - offset) / (dash_len * 2.0);
            (d.floor() as i32, d.fract())
        };
        for i in 0..full {
            let i = i as f32;
            // frame.stroke(0, 255, 0, 255);
            line!(offset + i * (dash_len * 2.0), dash_len);
        }
        let bit = (rem * dash_len * 2.0).min(dash_len);
        if bit > 0.0 {
            // frame.stroke(0, 0, 255, 255);
            line!(offset + (dash_len * 2.0) * full as f32, bit);
        }
        (1.0 - rem) * (dash_len * 2.0)
    }

    fn get_camera_world_rect(&self) -> Rect<f32> {
        let (cx, cy) = (self.camera_pos.x, self.camera_pos.y);
        let s = self.get_zoom_scale();
        Rect::new(
            cx - self.width as f32 / 2.0 / s,
            cy - self.height as f32 / 2.0 / s,
            self.width as f32 / s,
            self.height as f32 / s,
        )
    }
    fn get_viewable_chunks(&self) -> Vec<ChunkCoord> {
        let mut view_rect = self.get_camera_world_rect();
        view_rect.w /= 2.0;
        view_rect.h /= 2.0;
        view_rect.x += view_rect.w / 2.0;
        view_rect.y += view_rect.h / 2.0;

        let mut out = vec![];

        for x in ((view_rect.x / CHUNK_SIZE_UNITS as f32).floor() as i32)
            ..=(((view_rect.x + view_rect.w) / CHUNK_SIZE_UNITS as f32).floor() as i32)
        {
            for y in ((view_rect.y / CHUNK_SIZE_UNITS as f32).floor() as i32)
                ..=(((view_rect.y + view_rect.h) / CHUNK_SIZE_UNITS as f32).floor() as i32)
            {
                if LEVEL_RECT_BLOCKS.overlaps_excl(&chunk_rect_blocks(x, y)) {
                    out.push(ChunkCoord { x, y });
                }
            }
        }

        out
    }

    fn draw_text<T>(&mut self, frame: &Frame, text: T, x: f32, y: f32, size: f32)
    where
        T: ToString,
    {
        let v = frame.get_current_transform() * vector![x, y, 1.0];
        let (x, y) = (v.x, v.y);
        self.text_draws.push(TextDraw {
            text: text.to_string(),
            size,
            x,
            y,
        })
    }
}

impl CanvasAppState for State {
    fn init(loader: &mut desen::state::ResourceLoader) -> Self {
        // need this so that the atlas is nonzero lol!!!
        // technically not rly cause we are importing other stuff
        // but just in case
        // loader.load_texture_bytes(include_bytes!("../biddledoo.png"));
        Self {
            time: 0.0,
            width: 10,
            height: 10,
            camera_pos: vector![0.0, 0.0],
            zoom: 0.0,
            bg_color: (40, 125, 255, 255),
            background: loader.load_texture_bytes(include_bytes!("../background.png")),
            spritesheet: loader.load_texture_bytes(include_bytes!("../../src/gd/spritesheet.png")),
            level: Level::default(),
            preview_object: GDObject {
                id: 1,
                x: 15.0,
                y: 15.0,
                rotation: 0,
                flip_x: false,
                flip_y: false,
                scale: 1.0,
                z_layer: crate::layer::ZLayer::T1,
                z_order: 1,
                main_color: GDColor::white(),
                detail_color: GDColor::white(),
            },
            show_preview: false,
            select_depth: 0,
            selected_object: None,
            text_draws: vec![],
        }
    }
}

#[wasm_bindgen]
pub struct StateWrapper {
    bundle: CanvasAppBundle<State>,
}

impl StateWrapper {
    pub fn new(bundle: CanvasAppBundle<State>) -> Self {
        Self { bundle }
    }
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
        self.bundle.state.zoom = v.clamp(-36.0, 36.0);
    }
    pub fn get_zoom_scale(&self) -> f32 {
        self.bundle.state.get_zoom_scale()
    }

    pub fn get_world_pos(&self, x: f32, y: f32) -> Vec<f32> {
        let inv = self.bundle.state.view_transform().try_inverse().unwrap();

        let p = inv * vector![x, y, 1.0];

        vec![p.x, p.y]
    }

    pub fn add_object(&mut self, key: String, obj: GDObject) -> Result<(), RustError> {
        if get_object_info(obj.id as u32).is_none() {
            return ErrorType::InvalidObjectId(obj.id).into();
        }

        if let Ok(key) = key.into_bytes().try_into() {
            let key: DbKey = key;

            let chunk = obj.get_chunk_coord();

            self.bundle
                .state
                .level
                .chunks
                .entry(chunk)
                .or_insert_with(ChunkInfo::new)
                .objects
                .get_mut(obj.z_layer)
                .objects
                .entry(obj.z_order)
                .or_default()
                .insert(key, obj);
        }
        Ok(())
    }
    pub fn delete_object(&mut self, key: String) {
        if let Ok(key) = key.into_bytes().try_into() {
            let key: DbKey = key;

            if Some(key) == self.bundle.state.selected_object {
                self.bundle.state.selected_object = None;
            }

            for c in self.bundle.state.level.chunks.values_mut() {
                for (list, _) in c.objects.iter_mut() {
                    for m in list.objects.values_mut() {
                        if m.remove(&key).is_some() {
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn get_chunks_to_sub(&mut self) -> Vec<ChunkCoord> {
        let visible = self.bundle.state.get_viewable_chunks();

        let mut out = vec![];

        for v in visible {
            // check if it is in the hashmap already
            //if not, set it to subscribe
            match self.bundle.state.level.chunks.get_mut(&v) {
                Some(chunk) => chunk.last_time_visible = now(),
                None => {
                    self.bundle.state.level.chunks.insert(v, ChunkInfo::new());
                    out.push(v);
                }
            }
        }

        out
    }
    pub fn get_chunks_to_unsub(&mut self) -> Vec<ChunkCoord> {
        let mut out = vec![];
        let now = now();

        self.bundle.state.level.chunks.retain(|coord, chunk| {
            if now - chunk.last_time_visible > UNLOAD_CHUNK_TIME * 1000.0 {
                out.push(*coord);
                false
            } else {
                true
            }
        });

        out
    }

    pub fn set_preview_visibility(&mut self, to: bool) {
        self.bundle.state.show_preview = to;
    }
    pub fn get_preview_object(&mut self) -> GDObject {
        self.bundle.state.preview_object
    }
    pub fn set_preview_object(&mut self, to: GDObject) {
        self.bundle.state.preview_object = to
    }

    pub fn try_select_at(&mut self, x: f32, y: f32) {
        let chunk = get_chunk_coord(x, y);

        let mut clickable = vec![];

        for i in -1..=1 {
            for j in -1..=1 {
                let cx = chunk.x + i;
                let cy = chunk.y + j;
                self.bundle.state.level.foreach_obj_in_chunk(
                    ChunkCoord { x: cx, y: cy },
                    |key, obj| {
                        let rect = padded_obj_rect(obj, 20.0);

                        let t = obj_transform(obj);

                        let corners_world = rect.corners().map(|(x, y)| {
                            let v = t * vector![x, y, 1.0];
                            (v.x, v.y)
                        });

                        if point_in_triangle(
                            (x, y),
                            corners_world[0],
                            corners_world[1],
                            corners_world[2],
                        ) || point_in_triangle(
                            (x, y),
                            corners_world[0],
                            corners_world[2],
                            corners_world[3],
                        ) {
                            clickable.push(key);
                        }
                    },
                );
            }
        }

        self.bundle.state.selected_object = if clickable.is_empty() {
            None
        } else {
            if self.bundle.state.select_depth as usize >= clickable.len() {
                self.bundle.state.select_depth = 0;
            }
            self.bundle.state.select_depth += 1;
            Some(clickable[self.bundle.state.select_depth as usize - 1])
        };
    }
    pub fn deselect_object(&mut self) {
        self.bundle.state.selected_object = None;
    }
    pub fn get_selected_object_key(&mut self) -> Option<String> {
        self.bundle
            .state
            .selected_object
            .and_then(|v| String::from_utf8(v.into()).ok())
    }
    pub fn get_selected_object_chunk(&mut self) -> Option<ChunkCoord> {
        self.bundle.state.selected_object.and_then(|k| {
            self.bundle
                .state
                .level
                .get_obj_by_key(k)
                .map(|o| o.get_chunk_coord())
        })
    }
    pub fn get_text_draws(&self) -> Vec<TextDraw> {
        self.bundle.state.text_draws.clone()
    }
}

const UNLOAD_CHUNK_TIME: f64 = 1.0;
