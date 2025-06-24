use core::f64;
use std::sync::Arc;

use chrono::{DateTime, Local};
use glam::{ivec2, vec2, vec4, Affine2, IVec2, Vec2};

pub const DRAW_LEVEL: bool = true;

use rust_shared::{
    gd::{
        layer::ZLayer,
        level::{
            CHUNK_SIZE_BLOCKS, CHUNK_SIZE_UNITS, LEVEL_HEIGHT_UNITS, LEVEL_RECT_BLOCKS,
            LEVEL_WIDTH_UNITS,
        },
        object::{GDColor, GDObject},
        HitboxType, ObjectCategory,
    },
    util::{point_in_triangle, Rect},
};

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::{
    config::Config,
    error::AppError,
    level::{ChunkCoord, DbKey, Level},
    object::{GDObjectExt, GDObjectOpt},
    render::{
        data::Globals,
        rectdraw::{
            billy::{Billy, BlendMode},
            countdown::{Countdown, StatsDisplay},
        },
        state::RenderState,
    },
    utilgen::OBJECT_INFO,
    CustomWindow, RustError,
};

#[derive(Clone, Copy)]
pub struct EndingAnimInfo {
    pub initial_zoom: f32,
    pub initial_camera_pos: Vec2,
    pub initial_bg_color: (u8, u8, u8),
    pub initial_ground1_color: (u8, u8, u8),
    pub initial_ground2_color: (u8, u8, u8),
}

pub struct PendingState(Option<State>);

impl PendingState {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn init_state(
        &mut self,
        window: Arc<CustomWindow>,
        size: (u32, u32),
        config: Config,
    ) -> Result<(), AppError> {
        let mut partial_render_state =
            futures::executor::block_on(RenderState::new_canvas_partial(Arc::clone(&window), size))
                .unwrap();

        partial_render_state.clear_screen(&config);

        // window.set_visible(true);

        log::debug!("[CLOCK] upgrading state");

        let render_state = partial_render_state.upgrade(&config).unwrap();

        log::debug!("[CLOCK] finished upgrading state");

        self.0 = Some(State::new(render_state, size, config));

        Ok(())
    }

    pub fn ready(&mut self) -> Option<&mut State> {
        self.0.as_mut()
    }
}

pub struct State {
    pub(crate) render: RenderState,

    pub(crate) config: Config,

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

    pub(crate) now: DateTime<Local>,
    // // (text, x, y, lifetime)
    // delete_texts: Vec<(String, f32, f32, f32)>,

    // text_draws: Vec<TextDraw>,
    pub(crate) ending_anim_info: Option<EndingAnimInfo>,
    pub(crate) ending_transition_override: Option<f32>,
    pub(crate) ending_transition_speed: f32,
}

impl State {
    pub fn new(render: RenderState, size: (u32, u32), config: Config) -> Self {
        Self {
            time: 0.0,
            width: size.0,
            height: size.1,
            quality: 1.0,
            camera_pos: vec2(0.0, 0.0),
            zoom: config.general.zoom,
            bg_color: (
                config.background.image_tint.r,
                config.background.image_tint.g,
                config.background.image_tint.b,
            ),
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
            no_rotating_objects: true,
            hide_grid: config.grid.opacity == 0.0,
            hide_ground: false,
            hide_outline: false,
            select_hazards: false,
            event_start: f64::INFINITY,
            event_end: f64::INFINITY,
            ending_fully_done: f64::INFINITY,
            render,
            countdown: Countdown::new(&config),
            stats_display: StatsDisplay::new(),
            now: Local::now(),
            ending_anim_info: None,
            ending_transition_override: None,
            ending_transition_speed: 0.0,
            config,
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
        let gongy = Rect::new(
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
    fn get_viewable_chunks(&self) -> Vec<ChunkCoord> {
        let view_rect = self.get_camera_world_rect().expanded(1.5);

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
        self.zoom = v.clamp(-16.0, 36.0);
    }

    pub fn get_world_pos(&self, x: f32, y: f32) -> (f32, f32) {
        let inv = self.view_transform().inverse();

        let p = inv.transform_point2(vec2(x, y));

        (p.x, p.y)
    }
    pub fn get_screen_pos(&self, x: f32, y: f32) -> (f32, f32) {
        let p = self.view_transform().transform_point2(vec2(x, y));

        (p.x, p.y)
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

    // pub fn get_chunks_to_sub(&mut self) -> Vec<ChunkCoord> {
    //     let visible = self.get_viewable_chunks();

    //     let mut out = vec![];

    //     for v in visible {
    //         // check if it is in the hashmap already
    //         //if not, set it to subscribe
    //         match self.level.chunks.get_mut(&v) {
    //             Some(chunk) => chunk.last_time_visible = self.now,
    //             None => {
    //                 self.level.chunks.insert(v, LevelChunk::new(self.now));
    //                 out.push(v);
    //             }
    //         }
    //     }

    //     out
    // }
    // pub fn get_chunks_to_unsub(&mut self) -> Vec<ChunkCoord> {
    //     let mut out = vec![];

    //     self.level.chunks.retain(|coord, chunk| {
    //         if self.now - chunk.last_time_visible > UNLOAD_CHUNK_TIME * 1000.0 {
    //             out.push(*coord);
    //             false
    //         } else {
    //             true
    //         }
    //     });

    //     out
    // }

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

    pub fn set_now(&mut self, to: DateTime<Local>) {
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

    // pub fn set_stats(&mut self, num: u32) {
    //     self.stats_display.set_to(Some(num), self.now);
    // }

    // pub fn hide_stats(&mut self) {
    //     self.stats_display.set_to(None, self.now);
    // }

    pub fn compute_grid_offset(
        &self,
        screen_size: Vec2,
        zoom_scale: f32,
        grid_size: Vec2,
        edge_screen_pos: Vec2,
    ) -> Vec2 {
        let world_pos = ((edge_screen_pos - screen_size / 2.0) * vec2(1.0, -1.0)) / zoom_scale;

        let grid_aligned_world = vec2(
            (world_pos.x / grid_size.x).round() * grid_size.x,
            (world_pos.y / grid_size.y).round() * grid_size.y,
        );

        let screen_aligned_pos =
            (grid_aligned_world * zoom_scale) * vec2(1.0, -1.0) + screen_size / 2.0;

        return edge_screen_pos - screen_aligned_pos;
    }

    pub fn transition_to_ending_spectate(&mut self, duration: f32) {
        self.ending_transition_override = Some(1.0);
        self.ending_transition_speed = -(1.0 / duration);
    }

    fn render_inner(&mut self, delta: f32) -> Result<(), AppError> {
        let output = self
            .render
            .surface
            .get_current_texture()
            .map_err(AppError::SurfaceError)?;

        let output_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // let multisample_view = self
        //     .render
        //     .device
        //     .create_texture(&self.render.multisampled_frame_descriptor)
        //     .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .render
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let position = &self.config.clock.position;
        let grid_shift_world = match &position[..] {
            align @ ("top-left" | "top-right" | "bottom-left" | "bottom-right") => {
                let (edge_screen_pos, padding) = match align {
                    "top-left" => (
                        vec2(0.0, 0.0),
                        (
                            self.config.clock.padding.left,
                            self.config.clock.padding.top,
                        ),
                    ),
                    "top-right" => (
                        vec2(self.width as f32, 0.0),
                        (
                            self.config.clock.padding.right,
                            self.config.clock.padding.top,
                        ),
                    ),
                    "bottom-left" => (
                        vec2(0.0, self.height as f32),
                        (
                            self.config.clock.padding.left,
                            self.config.clock.padding.bottom,
                        ),
                    ),
                    "bottom-right" => (
                        vec2(self.width as f32, self.height as f32),
                        (
                            self.config.clock.padding.right,
                            self.config.clock.padding.bottom,
                        ),
                    ),
                    _ => unreachable!(),
                };

                let grid_shift_screen = self.compute_grid_offset(
                    vec2(self.width as f32, self.height as f32),
                    self.get_zoom_scale(),
                    vec2(30.0, 30.0),
                    edge_screen_pos,
                );

                let world = self.get_world_pos(grid_shift_screen.x, grid_shift_screen.y);

                (-(world.0 + padding.0 as f32), world.1 + padding.1 as f32)
            }

            // center
            _ => (15.0, 0.0),
        };

        self.render.queue.write_buffer(
            &self.render.globals_buffer,
            0,
            bytemuck::cast_slice(&[Globals {
                screen_size: [
                    self.render.surface_config.width as f32,
                    self.render.surface_config.height as f32,
                ],
                time: self.time,
                grid_opacity: self.config.grid.opacity / 255.0,
                camera_pos: self.camera_pos.to_array(),
                zoom_scale: self.get_zoom_scale(),
                _pad0: 0.0,
                grid_shift: [grid_shift_world.0, grid_shift_world.1],
                _pad_end: [0.0; 2],
            }]),
        );

        {
            self.time += delta;

            let mut billy = Billy::new();

            billy.set_blend_mode(BlendMode::Normal);

            // background
            let bg_tint = vec4(
                self.bg_color.0 as f32 / 255.0,
                self.bg_color.1 as f32 / 255.0,
                self.bg_color.2 as f32 / 255.0,
                self.config.background.image_tint.a as f32 / 255.0,
            );
            let bg_uv = vec2(self.render.bg_size.0 as f32, self.render.bg_size.1 as f32);

            match &self.config.background.fit[..] {
                "fill" => {
                    billy.centered_textured_rect(
                        vec2(0.0, 0.0),
                        vec2(self.width as f32, self.height as f32),
                        bg_tint,
                        0,
                        vec2(0.0, 0.0),
                        bg_uv,
                    );
                }
                fit @ ("contain" | "cover") => {
                    let img_width = self.render.bg_size.0 as f32;
                    let img_height = self.render.bg_size.1 as f32;
                    let win_width = self.width as f32;
                    let win_height = self.height as f32;

                    let img_aspect = img_width / img_height;
                    let win_aspect = win_width / win_height;

                    let scale_axis = if fit == "cover" {
                        img_aspect < win_aspect
                    } else {
                        img_aspect > win_aspect
                    };

                    let (bg_width, bg_height) = if scale_axis {
                        let scale = win_width / img_width;
                        (win_width, img_height * scale)
                    } else {
                        let scale = win_height / img_height;
                        (img_width * scale, win_height)
                    };

                    billy.centered_textured_rect(
                        vec2(0.0, 0.0),
                        vec2(bg_width, bg_height),
                        bg_tint,
                        0,
                        vec2(0.0, 0.0),
                        bg_uv,
                    )
                }
                "tile" => {
                    let old_t = billy.get_transform();
                    billy.translate(-self.camera_pos / 3.0 / 2.0);

                    let scale = f32::max(
                        self.width as f32 / self.render.bg_size.0 as f32,
                        self.height as f32 / self.render.bg_size.1 as f32,
                    ) / 600.0
                        * 1.5
                        * 1.25
                        * 400.0;

                    let scale = scale.min(2.0).max(1.0);

                    let tile_count_x =
                        (self.width as f32 / (self.render.bg_size.0 as f32 * scale)).ceil() as i32;
                    let tile_count_y =
                        (self.height as f32 / (self.render.bg_size.1 as f32 * scale)).ceil() as i32;

                    for x in (-tile_count_x / 2)..=(tile_count_x / 2) {
                        for y in (-tile_count_y / 2)..=(tile_count_y / 2) {
                            billy.centered_textured_rect(
                                vec2(
                                    (self.render.bg_size.0 as f32) * x as f32 * scale as f32,
                                    (self.render.bg_size.1 as f32) * y as f32 * scale as f32,
                                ),
                                vec2(
                                    self.render.bg_size.0 as f32 * scale,
                                    self.render.bg_size.1 as f32 * scale,
                                ),
                                bg_tint,
                                0,
                                vec2(0.0, 0.0),
                                bg_uv,
                            )
                        }
                    }

                    billy.set_transform(old_t);
                }
                "none" => billy.centered_textured_rect(
                    vec2(0.0, 0.0),
                    vec2(self.render.bg_size.0 as f32, self.render.bg_size.1 as f32),
                    bg_tint,
                    0,
                    vec2(0.0, 0.0),
                    bg_uv,
                ),
                "hidden" => (),
                _ => (),
            }

            let old_t = billy.get_transform();

            billy.apply_transform(self.view_transform());

            self.countdown
                .update_state(self.event_start, self.now, &self.config);
            self.countdown.draw(self, &mut billy);

            billy.set_transform(old_t);

            // these lines just commit the previous call
            billy.set_blend_mode(BlendMode::Additive);
            billy.set_blend_mode(BlendMode::Normal);

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
                        view: &self.render.multisample_view,
                        resolve_target: Some(&output_view),
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: self.config.background.back_color.r as f64 / 255.0,
                                g: self.config.background.back_color.g as f64 / 255.0,
                                b: self.config.background.back_color.b as f64 / 255.0,
                                a: self.config.background.back_color.a as f64 / 255.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

            render_pass.set_bind_group(0, &self.render.globals_bind_group, &[]);
            render_pass.set_bind_group(1, &self.render.textures_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.render.rect_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
            render_pass.set_index_buffer(
                self.render.rect_index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );

            let mut last_instance = 0;

            for (i, call) in billy.calls.iter().enumerate() {
                render_pass.set_pipeline(match call.blend_mode {
                    BlendMode::Normal => &self.render.pipeline_rect,
                    BlendMode::Additive => &self.render.pipeline_rect_additive_sq_alpha,
                });
                render_pass.draw_indexed(0..6, 0, last_instance..call.until_instance);
                last_instance = call.until_instance;

                // 1 because the first rect is the background and we want the grid on top
                if i == 1 {
                    render_pass.set_pipeline(&self.render.pipeline_grid);
                    render_pass.draw_indexed(0..6, 0, 0..1);
                }
            }
        }

        self.render.queue.submit([encoder.finish()]);
        output.present();

        Ok(())
    }

    pub fn render(&mut self, delta: f32) -> Result<(), AppError> {
        self.render_inner(delta)
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

//
// pub struct SelectedObjectInfo {
//     key: String,
//     pub id: u16,
//     pub main_color: GDColor,
//     pub detail_color: GDColor,
//     pub z_layer: ZLayer,
//     pub z_order: i8,
// }

//
// impl SelectedObjectInfo {
//     pub fn key(&self) -> String {
//         self.key.clone()
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct HitObjectInfo {
    pub(crate) key: DbKey,
    pub obj: GDObject,
}

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
