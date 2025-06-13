#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub screen_size: [f32; 2],
    pub quality: f32,
    pub grid_opacity: f32,
    pub camera_pos: [f32; 2],
    pub zoom_scale: f32,
    pub time: f32,
    // pub end_anim_time: f32,
}
