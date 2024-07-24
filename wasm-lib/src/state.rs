use glam::{mat2, uvec2, vec2, vec4, Affine2, Vec2, Vec4};
use the_nexus::packing::SpriteInfo;
use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;

use crate::{
    layer::{ZLayer, Z_LAYERS},
    level::{
        ChunkCoord, ChunkInfo, DbKey, Level, CHUNK_SIZE_BLOCKS, CHUNK_SIZE_UNITS,
        LEVEL_HEIGHT_BLOCKS, LEVEL_HEIGHT_UNITS, LEVEL_RECT_BLOCKS, LEVEL_WIDTH_BLOCKS,
        LEVEL_WIDTH_UNITS,
    },
    log, map,
    object::{GDColor, GDObject, GDObjectOpt},
    render::{data::Globals, pipeline_rect, state::RenderState},
    util::{get_chunk_coord, get_max_bounding_box, now, point_in_triangle, quick_image_load, Rect},
    utilgen::{DETAIL_SPRITES, MAIN_SPRITES, OBJECT_INFO},
    ErrorType, RustError,
};

#[wasm_bindgen]
pub struct State {
    render: RenderState,

    time: f32,

    width: u32,
    height: u32,

    camera_pos: Vec2,
    zoom: f32,

    level: Level,

    bg_color: (u8, u8, u8),
    ground1_color: (u8, u8, u8),
    ground2_color: (u8, u8, u8),

    preview_object: GDObjectOpt,
    show_preview: bool,

    selected_object: Option<DbKey>,
    select_depth: u32,
    // // (text, x, y, lifetime)
    // delete_texts: Vec<(String, f32, f32, f32)>,

    // text_draws: Vec<TextDraw>,
}

impl State {
    pub fn new(render: RenderState) -> Self {
        Self {
            time: 0.0,
            width: 10,
            height: 10,
            camera_pos: vec2(0.0, 0.0),
            zoom: 0.0,
            bg_color: (40, 125, 255),
            ground1_color: (40, 125, 255),
            ground2_color: (127, 178, 255),

            level: Level::default(),
            preview_object: GDObjectOpt {
                id: 1,
                x: 15.0,
                y: 15.0,
                x_scale_exp: 0,
                x_angle: 0,
                y_scale_exp: 0,
                y_angle: 18,
                z_layer: crate::layer::ZLayer::T1,
                z_order: 1,
                main_color: GDColor::white(),
                detail_color: GDColor::white(),
            },
            show_preview: false,
            select_depth: 0,
            selected_object: None,
            render,
        }
    }
    pub fn view_transform(&self) -> Affine2 {
        let scale = self.get_zoom_scale();

        Affine2::from_scale(vec2(scale, scale)) * Affine2::from_translation(-self.camera_pos)

        // Affine2::from_scale_angle_translation(vec2(scale, scale), 0.0, -self.camera_pos)
    }
}

fn obj_transform(obj: &GDObject) -> Affine2 {
    let scale = OBJECT_INFO[obj.id as usize].builtin_scale / 4.0;

    Affine2::from_mat2_translation(
        mat2(
            vec2(obj.ix * scale, obj.iy * scale),
            vec2(obj.jx * scale, obj.jy * scale),
        ),
        vec2(obj.x, obj.y),
    )

    // Matrix3::new(
    //     obj.ix * scale, obj.jx * scale, obj.x,
    //     obj.iy * scale, obj.jy * scale, obj.y,
    //     0.0, 0.0, 1.0
    // )
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
    // #[allow(clippy::too_many_arguments)]
    // fn dashed_line(
    //     &self,
    //     frame: &mut Frame,
    //     x0: f32,
    //     y0: f32,
    //     x1: f32,
    //     y1: f32,
    //     dash_len: f32,
    //     offset: f32,
    //     (r, g, b): (u8, u8, u8),
    //     weight: f32,
    // ) -> f32 {
    //     let start = vector![x0, y0];
    //     let end = vector![x1, y1];
    //     let to = end - start;
    //     let dir = to.normalize();

    //     macro_rules! line {
    //         ($a:expr, $l:expr) => {{
    //             let pb = start + dir * ($a + $l);
    //             let pa = start + dir * $a;
    //             frame.no_fill();
    //             frame.stroke(Color::Rgba8(r, g, b, 255));
    //             frame.stroke_weight(weight);
    //             frame.line().xy0(pa.x, pa.y).xy1(pb.x, pb.y);

    //             frame.no_stroke();
    //             frame.fill(Color::Rgba8(r, g, b, 255));
    //             frame.circle().xy(pa.x, pa.y).radius(weight / 2.0);
    //             frame.circle().xy(pb.x, pb.y).radius(weight / 2.0);
    //         }};
    //     }

    //     let offset = offset.rem_euclid(dash_len * 2.0);
    //     if offset > dash_len {
    //         let bit = offset - dash_len;

    //         // frame.stroke(255, 0, 0, 255);
    //         line!(0.0, bit);
    //     }
    //     let (full, rem) = {
    //         let d = (to.magnitude() - offset) / (dash_len * 2.0);
    //         (d.floor() as i32, d.fract())
    //     };
    //     for i in 0..full {
    //         let i = i as f32;
    //         // frame.stroke(0, 255, 0, 255);
    //         line!(offset + i * (dash_len * 2.0), dash_len);
    //     }
    //     let bit = (rem * dash_len * 2.0).min(dash_len);
    //     if bit > 0.0 {
    //         // frame.stroke(0, 0, 255, 255);
    //         line!(offset + (dash_len * 2.0) * full as f32, bit);
    //     }
    //     (1.0 - rem) * (dash_len * 2.0)
    // }

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

    // fn draw_text<T, U>(
    //     &mut self,
    //     frame: &Frame,
    //     text: T,
    //     x: f32,
    //     y: f32,
    //     font_size: f32,
    //     extra_style: U,
    // ) where
    //     T: ToString,
    //     U: ToString,
    // {
    //     let t = frame.get_transform().mat() * FrameTransform::Translate { x, y }.mat();

    //     self.text_draws.push(TextDraw {
    //         text: text.to_string(),
    //         font_size,
    //         transform: t,
    //         extra_style: extra_style.to_string(),
    //     })
    // }
}

#[wasm_bindgen]
impl State {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.render.resize(width, height);

        (self.width, self.height) = (width, height);
    }

    pub fn get_zoom_scale(&self) -> f32 {
        2.0f32.powf(self.zoom / 12.0)
    }
    pub fn get_camera_pos(&self) -> Vec<f32> {
        vec![self.camera_pos.x, self.camera_pos.y]
    }
    pub fn set_camera_pos(&mut self, x: f32, y: f32) {
        self.camera_pos = vec2(x, y).clamp(
            Vec2::ZERO,
            vec2(LEVEL_WIDTH_UNITS as f32, LEVEL_HEIGHT_UNITS as f32),
        );
    }
    pub fn set_bg_color(&mut self, r: u8, g: u8, b: u8) {
        self.bg_color = (r, g, b);
    }
    pub fn get_bg_color(&mut self) -> Vec<u8> {
        let (r, g, b) = self.bg_color;
        vec![r, g, b]
    }
    pub fn set_ground1_color(&mut self, r: u8, g: u8, b: u8) {
        self.ground1_color = (r, g, b);
    }
    pub fn get_ground1_color(&mut self) -> Vec<u8> {
        let (r, g, b) = self.ground1_color;
        vec![r, g, b]
    }
    pub fn set_ground2_color(&mut self, r: u8, g: u8, b: u8) {
        self.ground2_color = (r, g, b);
    }
    pub fn get_ground2_color(&mut self) -> Vec<u8> {
        let (r, g, b) = self.ground2_color;
        vec![r, g, b]
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
    pub fn set_zoom(&mut self, v: f32) {
        self.zoom = v.clamp(-36.0, 36.0);
    }

    pub fn get_world_pos(&self, x: f32, y: f32) -> Vec<f32> {
        let inv = self.view_transform().inverse();

        let p = inv.transform_point2(vec2(x, y));

        vec![p.x, p.y]
    }
    pub fn get_screen_pos(&self, x: f32, y: f32) -> Vec<f32> {
        let p = self.view_transform().transform_point2(vec2(x, y));

        vec![p.x, p.y]
    }

    pub fn add_object(&mut self, key: String, obj: GDObjectOpt) -> Result<(), RustError> {
        // if get_object_info(obj.id as u32).is_none() {
        //     return ErrorType::InvalidObjectId(obj.id).into();
        // }

        if let Ok(key) = key.into_bytes().try_into() {
            let key: DbKey = key;

            let chunk = obj.get_chunk_coord();

            self.level
                .chunks
                .entry(chunk)
                .or_insert_with(ChunkInfo::new)
                .objects
                .get_mut(obj.z_layer)
                .objects
                .entry(obj.z_order)
                .or_default()
                .insert(key, obj.into_obj());
        }
        Ok(())
    }
    pub fn delete_object(&mut self, key: String) -> Option<Vec<f32>> {
        if let Ok(key) = key.into_bytes().try_into() {
            let key: DbKey = key;

            if Some(key) == self.selected_object {
                self.selected_object = None;
            }

            for c in self.level.chunks.values_mut() {
                for (list, _) in c.objects.iter_mut() {
                    for m in list.objects.values_mut() {
                        if let Some(obj) = m.shift_remove(&key) {
                            return Some(vec![obj.x, obj.y]);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_chunks_to_sub(&mut self) -> Vec<ChunkCoord> {
        let visible = self.get_viewable_chunks();

        let mut out = vec![];

        for v in visible {
            // check if it is in the hashmap already
            //if not, set it to subscribe
            match self.level.chunks.get_mut(&v) {
                Some(chunk) => chunk.last_time_visible = now(),
                None => {
                    self.level.chunks.insert(v, ChunkInfo::new());
                    out.push(v);
                }
            }
        }

        out
    }
    pub fn get_chunks_to_unsub(&mut self) -> Vec<ChunkCoord> {
        let mut out = vec![];
        let now = now();

        self.level.chunks.retain(|coord, chunk| {
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
        self.show_preview = to;
    }
    pub fn get_preview_object(&mut self) -> GDObjectOpt {
        self.preview_object
    }
    pub fn set_preview_object(&mut self, mut to: GDObjectOpt) {
        to.fix();
        self.preview_object = to
    }
    pub fn is_preview_visible(&self) -> bool {
        self.show_preview
    }

    pub fn try_select_at(&mut self, x: f32, y: f32) -> Option<SelectedObjectInfo> {
        let chunk = get_chunk_coord(x, y);

        let mut clickable = vec![];

        for i in -1..=1 {
            for j in -1..=1 {
                let cx = chunk.x + i;
                let cy = chunk.y + j;
                self.level
                    .foreach_obj_in_chunk(ChunkCoord { x: cx, y: cy }, |key, obj| {
                        let rect = padded_obj_rect(obj, 20.0);

                        let t = obj_transform(obj);

                        let corners_world =
                            rect.corners().map(|(x, y)| t.transform_point2(vec2(x, y)));

                        if point_in_triangle(
                            vec2(x, y),
                            corners_world[0],
                            corners_world[1],
                            corners_world[2],
                        ) || point_in_triangle(
                            vec2(x, y),
                            corners_world[0],
                            corners_world[2],
                            corners_world[3],
                        ) {
                            clickable.push((key, *obj));
                        }
                    });
            }
        }

        let selected = if clickable.is_empty() {
            None
        } else {
            if self.select_depth as usize >= clickable.len() {
                self.select_depth = 0;
            }
            self.select_depth += 1;
            Some(clickable[self.select_depth as usize - 1])
        };

        self.selected_object = selected.map(|v| v.0);

        selected.map(|(key, obj)| SelectedObjectInfo {
            key: String::from_utf8(key.into()).unwrap(),
            id: obj.id,
            main_color: obj.main_color,
            detail_color: obj.detail_color,
            z_layer: obj.z_layer,
            z_order: obj.z_order,
        })
    }
    pub fn deselect_object(&mut self) {
        self.selected_object = None;
    }
    pub fn get_selected_object_key(&mut self) -> Option<String> {
        self.selected_object
            .and_then(|v| String::from_utf8(v.into()).ok())
    }
    pub fn get_selected_object_chunk(&mut self) -> Option<ChunkCoord> {
        self.selected_object
            .and_then(|k| self.level.get_obj_by_key(k).map(|o| o.get_chunk_coord()))
    }
    // pub fn get_text_draws(&self) -> Vec<TextDraw> {
    //     self.bundle.state.text_draws.clone()
    // }

    fn render_inner(&mut self, delta: f32) -> Result<(), wgpu::SurfaceError> {
        let output = self.render.surface.get_current_texture()?;
        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let multisample_view = self
            .render
            .device
            .create_texture(&self.render.multisampled_frame_descriptor)
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .render
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.render.queue.write_buffer(
            &self.render.globals_buffer,
            0,
            bytemuck::cast_slice(&[Globals {
                screen_size: [
                    self.render.surface_config.width as f32,
                    self.render.surface_config.height as f32,
                ],
                onion_size: self.render.onion_size.as_vec2().to_array(),
                camera_pos: self.camera_pos.to_array(),
                zoom_scale: self.get_zoom_scale(),
                // level_size: vec2(LEVEL_WIDTH_UNITS as f32, LEVEL_HEIGHT_UNITS as f32).to_array(),
                _pad: [0; 4],
            }]),
        );

        {
            self.time += delta;

            let mut rects: Vec<pipeline_rect::instance::Instance> = vec![];
            enum BlendMode {
                Normal,
                Additive,
            }
            struct DrawCall {
                blend_mode: BlendMode,
                until_instance: u32,
            }
            let mut calls: Vec<DrawCall> = vec![];

            // background
            {
                let mut transform = Affine2::IDENTITY;

                transform *= Affine2::from_translation(-self.camera_pos / 10.0);

                let scale = self.width.min(self.height) as f32 / 600.0 * 1.5 * 1.25 * 600.0;

                let offset = (self.camera_pos / 10.0 / scale).floor() * scale;

                for i in -2i32..=2 {
                    for j in -2i32..=2 {
                        let mut transform = transform;
                        transform *=
                            Affine2::from_translation(offset + scale * vec2(i as f32, j as f32));
                        transform *= Affine2::from_scale(vec2(
                            1.0,
                            if j.rem_euclid(2) == 1 { -1.0 } else { 1.0 },
                        ));

                        rects.push(pipeline_rect::instance::Instance::new(
                            transform
                                * Affine2::from_scale_angle_translation(
                                    vec2(scale, scale) + 1.0,
                                    0.0,
                                    -(vec2(scale, scale) + 1.0) / 2.0,
                                ),
                            // transform.transform_point2(-vec2(scale + 1.0, scale + 1.0) / 2.0),
                            // transform.transform_vector2(vec2(scale + 1.0, scale + 1.0)),
                            vec4(
                                self.bg_color.0 as f32 / 255.0,
                                self.bg_color.1 as f32 / 255.0,
                                self.bg_color.2 as f32 / 255.0,
                                1.0,
                            ),
                            1,
                            vec2(0.0, 0.0),
                            vec2(1024.0, 1024.0),
                        ));
                    }
                }

                calls.push(DrawCall {
                    blend_mode: BlendMode::Normal,
                    until_instance: rects.len() as u32,
                });
            };

            // objects
            {
                let transform = self.view_transform();

                let draw_obj_sprite = |rects: &mut Vec<pipeline_rect::instance::Instance>,
                                       mut transform: Affine2,
                                       sprite: SpriteInfo,
                                       obj: &GDObject,
                                       color: Vec4| {
                    transform *= obj_transform(obj);

                    let uv_pos = uvec2(sprite.pos.0, sprite.pos.1).as_vec2();
                    let uv_size = uvec2(sprite.size.0, sprite.size.1).as_vec2();

                    rects.push(pipeline_rect::instance::Instance::new(
                        transform
                            * Affine2::from_scale_angle_translation(
                                uv_size,
                                0.0,
                                vec2(
                                    -(sprite.size.0 as f32 / 2.0) + sprite.offset.0,
                                    -(sprite.size.1 as f32 / 2.0) - sprite.offset.1,
                                ),
                            ),
                        // transform.transform_point2(
                        //     -vec2(
                        //         sprite.size.0 as f32 / 2.0 + sprite.offset.0,
                        //         sprite.size.1 as f32 / 2.0 - sprite.offset.1,
                        //     ) / 2.0,
                        // ),
                        // transform.transform_vector2(uv_size),
                        color,
                        4,
                        uv_pos,
                        uv_size,
                    ));
                };

                let selected_color = |lighter| {
                    let c = map!(
                        (self.time * 10.0).sin(),
                        -1.0,
                        1.0,
                        if lighter {
                            150.0 / 255.0
                        } else {
                            100.0 / 255.0
                        },
                        if lighter { 1.0 } else { 200.0 / 255.0 }
                    );
                    vec4(1.0, c, c, 1.0)
                };

                for &layer in Z_LAYERS {
                    for z_order in -50..50 {
                        self.level.foreach_obj_in_z(
                            layer,
                            z_order,
                            |key, obj| {
                                if let Some(sprite) = MAIN_SPRITES[obj.id as usize] {
                                    if obj.main_color.blending {
                                        let color = if self.selected_object == Some(key) {
                                            selected_color(false)
                                        } else {
                                            Vec4::from_array(
                                                [
                                                    obj.main_color.r,
                                                    obj.main_color.g,
                                                    obj.main_color.b,
                                                    obj.main_color.opacity,
                                                ]
                                                .map(|v| v as f32 / 255.0),
                                            )
                                        };
                                        draw_obj_sprite(&mut rects, transform, sprite, obj, color);
                                    }
                                }
                            },
                            self.show_preview.then(|| self.preview_object.into_obj()),
                        )
                    }
                    calls.push(DrawCall {
                        blend_mode: BlendMode::Additive,
                        until_instance: rects.len() as u32,
                    });

                    for z_order in -50..50 {
                        self.level.foreach_obj_in_z(
                            layer,
                            z_order,
                            |key, obj| {
                                if let Some(sprite) = MAIN_SPRITES[obj.id as usize] {
                                    if !obj.main_color.blending {
                                        let color = if self.selected_object == Some(key) {
                                            selected_color(false)
                                        } else {
                                            Vec4::from_array(
                                                [
                                                    obj.main_color.r,
                                                    obj.main_color.g,
                                                    obj.main_color.b,
                                                    obj.main_color.opacity,
                                                ]
                                                .map(|v| v as f32 / 255.0),
                                            )
                                        };
                                        draw_obj_sprite(&mut rects, transform, sprite, obj, color);
                                    }
                                }
                                if let Some(sprite) = DETAIL_SPRITES[obj.id as usize] {
                                    if !obj.detail_color.blending {
                                        let color = if self.selected_object == Some(key) {
                                            selected_color(true)
                                        } else {
                                            Vec4::from_array(
                                                [
                                                    obj.detail_color.r,
                                                    obj.detail_color.g,
                                                    obj.detail_color.b,
                                                    obj.detail_color.opacity,
                                                ]
                                                .map(|v| v as f32 / 255.0),
                                            )
                                        };
                                        draw_obj_sprite(&mut rects, transform, sprite, obj, color);
                                    }
                                }
                            },
                            self.show_preview.then(|| self.preview_object.into_obj()),
                        )
                    }
                    calls.push(DrawCall {
                        blend_mode: BlendMode::Normal,
                        until_instance: rects.len() as u32,
                    });

                    for z_order in -50..50 {
                        self.level.foreach_obj_in_z(
                            layer,
                            z_order,
                            |key, obj| {
                                if let Some(sprite) = DETAIL_SPRITES[obj.id as usize] {
                                    if obj.detail_color.blending {
                                        let color = if self.selected_object == Some(key) {
                                            selected_color(false)
                                        } else {
                                            Vec4::from_array(
                                                [
                                                    obj.detail_color.r,
                                                    obj.detail_color.g,
                                                    obj.detail_color.b,
                                                    obj.detail_color.opacity,
                                                ]
                                                .map(|v| v as f32 / 255.0),
                                            )
                                        };
                                        draw_obj_sprite(&mut rects, transform, sprite, obj, color);
                                    }
                                }
                            },
                            self.show_preview.then(|| self.preview_object.into_obj()),
                        )
                    }
                    calls.push(DrawCall {
                        blend_mode: BlendMode::Additive,
                        until_instance: rects.len() as u32,
                    });
                }
            };

            // ground
            {
                const GROUND_SIZE_BLOCKS: f32 = 4.25;
                const GROUND_SIZE_UNITS: f32 = GROUND_SIZE_BLOCKS * 30.0;

                let transform = self.view_transform();

                let mut view_rect = self.get_camera_world_rect();
                // view_rect.w /= 2.0;
                // view_rect.h /= 2.0;
                // view_rect.x += view_rect.w / 2.0;
                // view_rect.y += view_rect.h / 2.0;
                let min_x = (view_rect.x / GROUND_SIZE_UNITS).floor() as i32 - 1;
                let max_x = ((view_rect.x + view_rect.w) / GROUND_SIZE_UNITS).floor() as i32 + 1;

                for i in min_x..=max_x {
                    let x = i as f32 * GROUND_SIZE_BLOCKS * 30.0;
                    let t = transform
                        * Affine2::from_scale_angle_translation(
                            vec2(GROUND_SIZE_UNITS, GROUND_SIZE_UNITS) + 0.2,
                            0.0,
                            vec2(x, -GROUND_SIZE_UNITS) - 0.1,
                        );
                    rects.push(pipeline_rect::instance::Instance::new(
                        t,
                        vec4(
                            self.ground1_color.0 as f32 / 255.0,
                            self.ground1_color.1 as f32 / 255.0,
                            self.ground1_color.2 as f32 / 255.0,
                            1.0,
                        ),
                        2,
                        vec2(0.0, 0.0),
                        vec2(256.0, 256.0),
                    ));
                    rects.push(pipeline_rect::instance::Instance::new(
                        t,
                        vec4(
                            self.ground2_color.0 as f32 / 255.0,
                            self.ground2_color.1 as f32 / 255.0,
                            self.ground2_color.2 as f32 / 255.0,
                            1.0,
                        ),
                        3,
                        vec2(0.0, 0.0),
                        vec2(256.0, 256.0),
                    ));
                }

                calls.push(DrawCall {
                    blend_mode: BlendMode::Normal,
                    until_instance: rects.len() as u32,
                });
            };
            let instance_buffer =
                self.render
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(&rects),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &multisample_view,
                    resolve_target: Some(&output_view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_bind_group(0, &self.render.globals_bind_group, &[]);
            render_pass.set_bind_group(1, &self.render.onion_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.render.rect_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.set_index_buffer(
                self.render.rect_index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            let mut last_instance = 0;
            for (i, call) in calls.iter().enumerate() {
                render_pass.set_pipeline(match call.blend_mode {
                    BlendMode::Normal => &self.render.pipeline_rect,
                    BlendMode::Additive => &self.render.pipeline_rect_additive_sq_alpha,
                });
                render_pass.draw_indexed(0..6, 0, last_instance..call.until_instance);
                last_instance = call.until_instance;

                if i == 0 {
                    render_pass.set_pipeline(&self.render.pipeline_grid);
                    render_pass.draw_indexed(0..6, 0, 0..1);
                }
            }
        }

        self.render.queue.submit([encoder.finish()]);
        output.present();

        Ok(())
    }

    pub fn render(&mut self, delta: f32) {
        self.render_inner(delta).unwrap()
    }
}

const UNLOAD_CHUNK_TIME: f64 = 1.0;

#[wasm_bindgen]
pub struct SelectedObjectInfo {
    key: String,
    pub id: u16,
    pub main_color: GDColor,
    pub detail_color: GDColor,
    pub z_layer: ZLayer,
    pub z_order: i8,
}

#[wasm_bindgen]
impl SelectedObjectInfo {
    pub fn key(&self) -> String {
        self.key.clone()
    }
}
