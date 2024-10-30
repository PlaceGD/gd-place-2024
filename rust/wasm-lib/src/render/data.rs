#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub screen_size: [f32; 2],
    pub onion_size: [f32; 2],
    pub camera_pos: [f32; 2],
    pub zoom_scale: f32,
    pub time: f32,
    // pub end_anim_time: f32,

    // pub padding: [f32; 2],
}
