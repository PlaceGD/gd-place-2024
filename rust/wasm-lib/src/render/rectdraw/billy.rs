use glam::{vec2, Affine2, Vec2, Vec4};

use crate::render::pipeline_rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Additive,
}
#[derive(Debug, Clone, Copy)]
pub struct DrawCall {
    pub blend_mode: BlendMode,
    pub until_instance: u32,
}

pub struct Billy {
    pub rects: Vec<pipeline_rect::instance::Instance>,
    pub calls: Vec<DrawCall>,
    blend_mode: BlendMode,
    transform: Affine2,
}
impl Billy {
    pub fn new() -> Self {
        Self {
            rects: Vec::with_capacity(1 << 19),
            calls: Vec::with_capacity(16),
            blend_mode: BlendMode::Additive,
            transform: Affine2::IDENTITY,
        }
    }

    #[inline]
    pub fn scale(&mut self, scale: Vec2) {
        self.transform *= Affine2::from_scale(scale);
    }
    #[inline]
    pub fn translate(&mut self, translation: Vec2) {
        self.transform *= Affine2::from_translation(translation);
    }
    #[inline]
    pub fn rotate(&mut self, angle: f32) {
        self.transform *= Affine2::from_angle(angle);
    }
    #[inline]
    pub fn apply_transform(&mut self, t: Affine2) {
        self.transform *= t;
    }
    #[inline]
    pub fn get_transform(&self) -> Affine2 {
        self.transform
    }
    #[inline]
    pub fn set_transform(&mut self, t: Affine2) {
        self.transform = t;
    }
    #[inline]
    pub fn solid_rect(&mut self, pos: Vec2, size: Vec2, color: Vec4) {
        self.rects.push(pipeline_rect::instance::Instance::new(
            self.transform * Affine2::from_scale_angle_translation(size, 0.0, pos),
            color,
            0,
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
        ));
    }
    #[inline]
    pub fn textured_rect(
        &mut self,
        pos: Vec2,
        size: Vec2,
        tint: Vec4,
        tex_idx: u32,
        uv_pos: Vec2,
        uv_size: Vec2,
    ) {
        self.rects.push(pipeline_rect::instance::Instance::new(
            self.transform * Affine2::from_scale_angle_translation(size, 0.0, pos),
            tint,
            tex_idx + 1,
            uv_pos,
            uv_size,
        ));
    }
    #[inline]
    pub fn dashed_rect(&mut self, pos: Vec2, size: Vec2, color: Vec4, uv_size: Vec2) {
        self.rects.push(pipeline_rect::instance::Instance::new(
            self.transform * Affine2::from_scale_angle_translation(size, 0.0, pos),
            color,
            1000,
            vec2(0.0, 0.0),
            uv_size,
        ));
    }
    #[inline]
    pub fn centered_solid_rect(&mut self, pos: Vec2, size: Vec2, color: Vec4) {
        self.solid_rect(pos - size / 2.0, size, color);
    }
    #[inline]
    pub fn centered_textured_rect(
        &mut self,
        pos: Vec2,
        size: Vec2,
        tint: Vec4,
        tex_idx: u32,
        uv_pos: Vec2,
        uv_size: Vec2,
    ) {
        self.textured_rect(pos - size / 2.0, size, tint, tex_idx, uv_pos, uv_size)
    }
    #[inline]
    pub fn centered_dashed_rect(&mut self, pos: Vec2, size: Vec2, color: Vec4, uv_size: Vec2) {
        self.dashed_rect(pos - size / 2.0, size, color, uv_size);
    }
    #[inline]
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        if blend_mode != self.blend_mode {
            self.calls.push(DrawCall {
                blend_mode: self.blend_mode,
                until_instance: self.rects.len() as u32,
            });
            self.blend_mode = blend_mode;
        }
    }
}
