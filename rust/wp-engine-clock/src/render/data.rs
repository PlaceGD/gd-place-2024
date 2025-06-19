#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub screen_size: [f32; 2],
    pub time: f32,
    pub grid_opacity: f32,
    pub camera_pos: [f32; 2],
    pub zoom_scale: f32,
    pub _pad0: f32, // 4 (align to 8)
    pub grid_shift: [f32; 2],
    pub _pad_end: [f32; 2],
}
