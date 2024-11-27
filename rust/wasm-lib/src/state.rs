use core::f64;
use std::{collections::HashSet, io::Cursor, sync::LazyLock};

use binrw::BinRead;
use glam::{mat2, uvec2, vec2, vec4, Affine2, Vec2, Vec4};

pub const DRAW_LEVEL: bool = true;

use rust_shared::{
    console_log,
    gd::{
        layer::ZLayer,
        level::{
            CHUNK_SIZE_BLOCKS, CHUNK_SIZE_UNITS, LEVEL_HEIGHT_UNITS, LEVEL_RECT_BLOCKS,
            LEVEL_WIDTH_UNITS,
        },
        object::{GDColor, GDObject},
        HitboxType, ObjectCategory,
    },
    history::{History, HistoryItem},
    map,
    util::{point_in_triangle, Rect},
};
use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;

pub static HISTORY: LazyLock<History> = LazyLock::new(|| {
    let bytes = include_bytes!("history");

    History::read(&mut Cursor::new(bytes)).unwrap()
});

use crate::{
    level::{ChunkCoord, DbKey, Level, LevelChunk},
    object::{GDObjectExt, GDObjectOpt},
    render::{
        data::Globals,
        rectdraw::{
            billy::{Billy, BlendMode},
            countdown::{Countdown, StatsDisplay},
            level::draw as level_draw,
        },
        state::RenderState,
    },
    utilgen::OBJECT_INFO,
    RustError,
};

#[derive(Clone, Copy)]
pub struct EndingAnimInfo {
    pub initial_zoom: f32,
    pub initial_camera_pos: Vec2,
    pub initial_bg_color: (u8, u8, u8),
    pub initial_ground1_color: (u8, u8, u8),
    pub initial_ground2_color: (u8, u8, u8),
}

#[wasm_bindgen]
pub struct State {
    pub(crate) render: RenderState,

    pub(crate) time: f32,

    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) quality: f32,

    pub(crate) camera_pos: Vec2,
    pub(crate) zoom: f32,

    pub(crate) level: Level<DbKey>,

    pub(crate) bg_color: (u8, u8, u8),
    pub(crate) ground1_color: (u8, u8, u8),
    pub(crate) ground2_color: (u8, u8, u8),

    pub(crate) preview_object: GDObjectOpt,
    pub(crate) show_preview: bool,

    pub(crate) selected_object: Option<DbKey>,
    // select_depth: u32,
    pub(crate) show_collidable: bool,
    pub(crate) hide_triggers: bool,
    pub(crate) no_rotating_objects: bool,
    pub(crate) hide_grid: bool,
    pub(crate) hide_ground: bool,
    pub(crate) hide_outline: bool,
    pub(crate) select_hazards: bool,

    /// unix time, negative before event starts
    //pub(crate) event_elapsed: f64,
    pub(crate) event_start: f64,

    pub(crate) event_end: f64,
    pub(crate) ending_fully_done: f64,

    pub(crate) countdown: Countdown,
    pub(crate) stats_display: StatsDisplay,

    pub(crate) now: f64,
    // // (text, x, y, lifetime)
    // delete_texts: Vec<(String, f32, f32, f32)>,

    // text_draws: Vec<TextDraw>,
    pub(crate) ending_anim_info: Option<EndingAnimInfo>,
    pub(crate) ending_transition_override: Option<f32>,
    pub(crate) ending_transition_speed: f32,
}

impl State {
    pub fn new(render: RenderState) -> Self {
        Self {
            time: 0.0,
            width: 10,
            height: 10,
            quality: 1.0,
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
                z_layer: ZLayer::T1,
                z_order: 1,
                main_color: GDColor::white(),
                detail_color: GDColor::white(),
            },
            show_preview: false,
            // select_depth: 0,
            selected_object: None,
            show_collidable: false,
            hide_triggers: false,
            no_rotating_objects: false,
            hide_grid: false,
            hide_ground: false,
            hide_outline: false,
            select_hazards: false,
            event_start: f64::INFINITY,
            event_end: f64::INFINITY,
            ending_fully_done: f64::INFINITY,
            render,
            countdown: Countdown::new(),
            stats_display: StatsDisplay::new(),
            now: js_sys::Date::now(), // default to client now before server now is gotten
            ending_anim_info: None,
            ending_transition_override: None,
            ending_transition_speed: 0.0,
        }
    }
    pub fn view_transform(&self) -> Affine2 {
        let scale = self.get_zoom_scale();
        // let size_scale = (self.)

        Affine2::from_scale(vec2(scale, scale)) * Affine2::from_translation(-self.camera_pos)

        // Affine2::from_scale_angle_translation(vec2(scale, scale), 0.0, -self.camera_pos)
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

    pub(crate) fn get_camera_world_rect(&self) -> Rect<f32> {
        let (cx, cy) = (self.camera_pos.x, self.camera_pos.y);
        let s = self.get_zoom_scale();
        let mut gongy = Rect::new(
            cx - self.width as f32 / 2.0 / s,
            cy - self.height as f32 / 2.0 / s,
            self.width as f32 / s,
            self.height as f32 / s,
        );

        // gongy.w /= 2.0;
        // gongy.h /= 2.0;
        // gongy.x += gongy.w / 2.0;
        // gongy.y += gongy.h / 2.0;

        gongy
    }
    pub fn get_viewable_chunks(&self) -> Vec<ChunkCoord> {
        let mut view_rect = self.get_camera_world_rect().expanded(1.5);

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
}

#[wasm_bindgen]
impl State {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.render.resize(width, height, self.quality);

        (self.width, self.height) = (width, height);
    }
    pub fn set_quality(&mut self, quality: f32) {
        self.render.resize(self.width, self.height, quality);

        self.quality = quality;
    }

    pub fn get_zoom_scale(&self) -> f32 {
        let size_zoom = (self.width as f32 / 1600.0).max(self.height as f32 / 900.0);

        2.0f32.powf(self.zoom / 12.0) * size_zoom
    }
    pub fn get_camera_pos(&self) -> Vec<f32> {
        vec![self.camera_pos.x, self.camera_pos.y]
    }
    pub fn set_camera_pos(&mut self, x: f32, y: f32) {
        self.camera_pos = vec2(x, y).clamp(
            vec2(-90.0, -60.0),
            vec2(
                LEVEL_WIDTH_UNITS as f32 + 60.0,
                LEVEL_HEIGHT_UNITS as f32 + 60.0,
            ),
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
        self.zoom = v.clamp(-64.0, 36.0);
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
            // let key: DbKey = key;

            self.level.add_object(obj.into_obj(), key, None, self.now);
        }
        Ok(())
    }
    pub fn delete_object(&mut self, key: String) -> Option<Vec<f32>> {
        if let Ok(key) = key.into_bytes().try_into() {
            let key: DbKey = key;

            if Some(key) == self.selected_object {
                self.selected_object = None;
            }

            if let Some(obj) = self.level.remove_object(key) {
                return Some(vec![obj.x, obj.y]);
            }

            // for c in self.level.chunks.values_mut() {
            //     for (list, _) in c.objects.iter_mut() {
            //         for m in list.objects.values_mut() {
            //             if let Some(obj) = m.shift_remove(&key) {
            //                 return Some(vec![obj.x, obj.y]);
            //             }
            //         }
            //     }
            // }
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
                Some(chunk) => chunk.last_time_visible = self.now,
                None => {
                    self.level.chunks.insert(v, LevelChunk::new(self.now));
                    out.push(v);
                }
            }
        }

        out
    }
    pub fn get_chunks_to_unsub(&mut self) -> Vec<ChunkCoord> {
        let mut out = vec![];

        self.level.chunks.retain(|coord, chunk| {
            if self.now - chunk.last_time_visible > UNLOAD_CHUNK_TIME * 1000.0 {
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

    pub fn objects_hit_at(&self, x: f32, y: f32, pad: f32) -> Vec<HitObjectInfo> {
        let chunk = ChunkCoord::get_from_pos(x, y);

        let mut clickable = vec![];

        for i in -1..=1 {
            for j in -1..=1 {
                let cx = chunk.x + i;
                let cy = chunk.y + j;
                self.level
                    .foreach_obj_in_chunk(ChunkCoord { x: cx, y: cy }, |key, obj| {
                        let info = OBJECT_INFO[obj.id as usize];

                        if self.show_collidable && info.hitbox_type == HitboxType::NoHitbox {
                            return;
                        }
                        if self.hide_grid && info.category == ObjectCategory::Triggers {
                            return;
                        }
                        if self.select_hazards
                            && ![HitboxType::Hazard, HitboxType::Solid].contains(&info.hitbox_type)
                        {
                            return;
                        }

                        let rect = obj.padded_rect(pad);

                        let t = obj.transform();

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
                            clickable.push(HitObjectInfo { key, obj: *obj });
                        }
                    });
            }
        }
        clickable
    }

    // pub fn try_select_at(&mut self, x: f32, y: f32) -> Option<SelectedObjectInfo> {
    //     let clickable = self.objects_hit_at(x, y, 20.0);

    //     let selected = if clickable.is_empty() {
    //         None
    //     } else {
    //         if self.select_depth as usize >= clickable.len() {
    //             self.select_depth = 0;
    //         }
    //         self.select_depth += 1;
    //         Some(clickable[self.select_depth as usize - 1])
    //     };

    //     self.selected_object = selected.map(|v| v.0);

    //     selected.map(|(key, obj)| SelectedObjectInfo {
    //         key: String::from_utf8(key.into()).unwrap(),
    //         id: obj.id,
    //         main_color: obj.main_color,
    //         detail_color: obj.detail_color,
    //         z_layer: obj.z_layer,
    //         z_order: obj.z_order,
    //     })
    // }
    pub fn deselect_object(&mut self) {
        self.selected_object = None;
    }
    pub fn set_selected_object(&mut self, key: String) {
        self.selected_object = Some(key.into_bytes().try_into().unwrap());
    }
    pub fn get_selected_object_key(&mut self) -> Option<String> {
        self.selected_object
            .and_then(|v| String::from_utf8(v.into()).ok())
    }
    pub fn get_selected_object_chunk(&mut self) -> Option<ChunkCoord> {
        self.selected_object.and_then(|k| {
            self.level
                .get_obj_by_key(k)
                .map(|o| ChunkCoord::get_from_pos(o.x, o.y))
        })
    }

    pub fn run_history(&mut self, index: usize, timelapse_time: u32) -> usize {
        let mut history_index = index;
        let mut removed_objs = HashSet::default();
        if history_index > 0 && HISTORY.actions[history_index - 1].time() > timelapse_time as u32 {
            while history_index > 0
                && HISTORY.actions[history_index - 1].time() < timelapse_time as u32
            {
                history_index -= 1;
                todo!()
            }
        } else {
            while history_index < HISTORY.actions.len()
                && HISTORY.actions[history_index].time() < timelapse_time as u32
            {
                let HistoryItem { obj, objkey, .. } = HISTORY.actions[history_index];
                if obj == [0; 26] {
                    removed_objs.insert(objkey);
                } else {
                    let nobj = GDObjectOpt::from_bytes(obj.as_slice().into())
                        .unwrap()
                        .into_obj();
                    if !((nobj.ix.powi(2) + nobj.iy.powi(2)).sqrt() > 3.0
                        || (nobj.jx.powi(2) + nobj.jy.powi(2)).sqrt() > 3.0)
                    {
                        self.level.add_object(nobj, objkey, None, self.now);
                    }
                }

                history_index += 1;
            }
        }

        self.level.remove_objects(removed_objs);

        //console_log!("{}", history_index - index);

        history_index
    }
    // pub fn get_text_draws(&self) -> Vec<TextDraw> {
    //     self.bundle.state.text_draws.clone()
    // }

    pub fn set_show_collidable(&mut self, to: bool) {
        self.show_collidable = to;
    }
    pub fn set_hide_triggers(&mut self, to: bool) {
        self.hide_triggers = to;
    }
    pub fn set_no_rotating_objects(&mut self, to: bool) {
        self.no_rotating_objects = to;
    }
    pub fn set_hide_grid(&mut self, to: bool) {
        self.hide_grid = to;
    }
    pub fn set_hide_outline(&mut self, to: bool) {
        self.hide_outline = to;
    }
    pub fn set_hide_ground(&mut self, to: bool) {
        self.hide_ground = to;
    }
    pub fn set_select_hazards(&mut self, to: bool) {
        self.select_hazards = to;
    }

    pub fn set_now(&mut self, to: f64) {
        self.now = to;
    }

    pub fn set_event_start(&mut self, to: f64) {
        self.event_start = to;
    }

    pub fn set_event_end(&mut self, to: f64) {
        self.event_end = to;
    }
    pub fn set_ending_fully_done(&mut self, to: f64) {
        self.ending_fully_done = to;
    }

    pub fn set_stats(&mut self, num: u32) {
        self.stats_display.set_to(Some(num), self.now);
    }

    pub fn hide_stats(&mut self) {
        self.stats_display.set_to(None, self.now);
    }

    pub fn transition_to_ending_spectate(&mut self, duration: f32) {
        self.ending_transition_override = Some(1.0);
        self.ending_transition_speed = -(1.0 / duration);
    }

    fn render_inner(&mut self, delta: f32, end_trans01: f32) -> Result<(), wgpu::SurfaceError> {
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

        if let Some(d) = &mut self.ending_transition_override {
            *d = (*d + self.ending_transition_speed * delta).clamp(0.0, 1.0);
        }

        self.render.queue.write_buffer(
            &self.render.globals_buffer,
            0,
            bytemuck::cast_slice(&[Globals {
                screen_size: [
                    self.render.surface_config.width as f32,
                    self.render.surface_config.height as f32,
                ],
                quality: self.quality,
                _unused: 0.0,
                camera_pos: self.camera_pos.to_array(),
                zoom_scale: self.get_zoom_scale(),
                // level_size: vec2(LEVEL_WIDTH_UNITS as f32, LEVEL_HEIGHT_UNITS as f32).to_array(),
                time: if end_trans01 == 0.0 {
                    self.time
                } else {
                    -end_trans01 as f32
                },
                // end_anim_time,
                // padding: [0.0; 2],
            }]),
        );

        {
            self.time += delta;

            let mut billy = Billy::new();

            // background
            {
                let old_t = billy.get_transform();
                billy.translate(-self.camera_pos / 3.0 / 2.0);

                let scale = self.width.min(self.height) as f32 / 600.0 * 1.5 * 1.25 * 600.0;

                let offset = (self.camera_pos / 10.0 / scale).floor() * scale;

                for i in -2i32..=2 {
                    for j in -2i32..=2 {
                        billy.centered_textured_rect(
                            offset + scale * vec2(i as f32, j as f32),
                            vec2(scale, scale),
                            vec4(
                                self.bg_color.0 as f32 / 255.0,
                                self.bg_color.1 as f32 / 255.0,
                                self.bg_color.2 as f32 / 255.0,
                                1.0,
                            ),
                            0,
                            vec2(0.0, 0.0),
                            vec2(1024.0, 1024.0),
                        );
                    }
                }
                billy.set_transform(old_t);
            };

            billy.set_blend_mode(BlendMode::Normal);
            //if self.event_elapsed < 0.0 {
            let old_t = billy.get_transform();
            billy.apply_transform(self.view_transform());
            self.countdown.update_state(self.event_start, self.now);
            self.countdown.draw(self, &mut billy);
            billy.set_transform(old_t);
            //}

            level_draw(self, &mut billy);

            // these lines just commit the previous call
            billy.set_blend_mode(BlendMode::Additive);
            billy.set_blend_mode(BlendMode::Normal);

            if self.now > self.event_end {
                let new_t = billy.get_transform();
                billy.set_transform(old_t);
                self.stats_display.draw(self, &mut billy);
                billy.set_transform(new_t);
            }

            let instance_buffer =
                self.render
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(&billy.rects),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

            let mut render_pass: wgpu::RenderPass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            render_pass.set_bind_group(1, &self.render.textures_bind_group, &[]);
            // render_pass.set_bind_group(2, &self.render.onion_nearest_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.render.rect_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.set_index_buffer(
                self.render.rect_index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            let mut last_instance = 0;
            //console_log!("CALLS {}", billy.calls.len());
            for (i, call) in billy.calls.iter().enumerate() {
                render_pass.set_pipeline(match call.blend_mode {
                    BlendMode::Normal => &self.render.pipeline_rect,
                    BlendMode::Additive => &self.render.pipeline_rect_additive_sq_alpha,
                });
                render_pass.draw_indexed(0..6, 0, last_instance..call.until_instance);
                last_instance = call.until_instance;

                if i == 0 && !self.hide_grid {
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
        fn ease_in_out_quart(x: f32) -> f32 {
            if x < 0.5 {
                8.0 * x * x * x * x
            } else {
                1.0 - (-2.0 * x + 2.0).powf(4.0) / 2.0
            }
        }

        fn ease_out_sine(x: f32) -> f32 {
            f32::sin(x * std::f32::consts::PI / 2.0)
        }

        let end_anim_time = ((self.now - self.event_end) / 1000.0) as f32;

        if end_anim_time > 0.0 && self.now < self.ending_fully_done {
            // console_log!("funny");
            if self.ending_anim_info.is_none() {
                self.ending_anim_info = Some(EndingAnimInfo {
                    initial_zoom: self.zoom,
                    initial_camera_pos: self.camera_pos,
                    initial_bg_color: self.bg_color,
                    initial_ground1_color: self.ground1_color,
                    initial_ground2_color: self.ground2_color,
                });
            }

            let EndingAnimInfo {
                initial_zoom,
                initial_camera_pos,
                initial_bg_color,
                initial_ground1_color,
                initial_ground2_color,
            } = self.ending_anim_info.unwrap();

            let zoomout_d = ease_out_sine((end_anim_time / 30.0).clamp(0.0, 1.0));
            self.zoom = map!(zoomout_d, 0.0, 1.0, initial_zoom, -2.0);
            // console_log!("zoom: {}; {}; {}", old_zoom, self.zoom, zoomout_d);

            let margin = vec2(40.0, 40.0) * 30.0;
            let cam_move_d = ease_in_out_quart((end_anim_time / 10.0).clamp(0.0, 1.0));
            let lerped_x = if initial_camera_pos.x < margin.x {
                map!(cam_move_d, 0.0, 1.0, initial_camera_pos.x, margin.x)
            } else if initial_camera_pos.x > LEVEL_WIDTH_UNITS as f32 - margin.x {
                map!(
                    cam_move_d,
                    0.0,
                    1.0,
                    initial_camera_pos.x,
                    LEVEL_WIDTH_UNITS as f32 - margin.x
                )
            } else {
                initial_camera_pos.x
            };

            let lerped_y = if initial_camera_pos.y < margin.y {
                map!(cam_move_d, 0.0, 1.0, initial_camera_pos.y, margin.y)
            } else if initial_camera_pos.y > LEVEL_HEIGHT_UNITS as f32 - margin.y {
                map!(
                    cam_move_d,
                    0.0,
                    1.0,
                    initial_camera_pos.y,
                    LEVEL_HEIGHT_UNITS as f32 - margin.y
                )
            } else {
                initial_camera_pos.y
            };
            self.camera_pos = vec2(lerped_x, lerped_y);

            // let color_d = 1.0 - (end_anim_time - 3.0 / 7.0).clamp(0.0, 1.0);

            // // self.bg_color = (
            // //     (initial_bg_color.0 as f32 * color_d) as u8,
            // //     (initial_bg_color.1 as f32 * color_d) as u8,
            // //     (initial_bg_color.2 as f32 * color_d) as u8,
            // // );

            // self.ground1_color = (
            //     (initial_ground1_color.0 as f32 * color_d) as u8,
            //     (initial_ground1_color.1 as f32 * color_d) as u8,
            //     (initial_ground1_color.2 as f32 * color_d) as u8,
            // );

            // self.ground2_color = (
            //     (initial_ground2_color.0 as f32 * color_d) as u8,
            //     (initial_ground2_color.1 as f32 * color_d) as u8,
            //     (initial_ground2_color.2 as f32 * color_d) as u8,
            // );
        } else {
            self.ending_anim_info = None;
        }

        self.render_inner(delta, get_end_trans01(self, end_anim_time))
            .unwrap();

        // (
        //     self.camera_pos,
        //     self.zoom,
        //     self.bg_color,
        //     self.ground1_color,
        //     self.ground2_color,
        // ) = (
        //     old_camera_pos,
        //     old_zoom,
        //     old_bg_color,
        //     old_ground1_color,
        //     old_ground2_color,
        // );
    }

    pub fn get_countdown_creator_names(&self) -> Vec<String> {
        self.countdown
            .sets
            .map(|i| rust_shared::countdown::get_creator_name(i).into())
            .into_iter()
            .collect()
    }
}

const UNLOAD_CHUNK_TIME: f64 = 1.0;

// #[wasm_bindgen]
// pub struct SelectedObjectInfo {
//     key: String,
//     pub id: u16,
//     pub main_color: GDColor,
//     pub detail_color: GDColor,
//     pub z_layer: ZLayer,
//     pub z_order: i8,
// }

// #[wasm_bindgen]
// impl SelectedObjectInfo {
//     pub fn key(&self) -> String {
//         self.key.clone()
//     }
// }

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct HitObjectInfo {
    pub(crate) key: DbKey,
    pub obj: GDObject,
}
#[wasm_bindgen]
impl HitObjectInfo {
    pub fn key(&self) -> String {
        String::from_utf8(self.key.into()).unwrap()
    }
}

pub fn get_end_trans01(state: &State, end_anim_time: f32) -> f32 {
    if let Some(d) = state.ending_transition_override {
        d * d
    } else {
        ((end_anim_time - 10.0) / 10.0).max(0.0)
    }
}
