#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpriteInfo {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub rotated: bool,
    pub offset: (f32, f32),
}

impl SpriteInfo {
    pub fn offset_rect_size(self) -> (f32, f32) {
        (
            self.size.0 as f32 + (self.offset.0 * 2.0).abs(),
            self.size.1 as f32 + (self.offset.1 * 2.0).abs(),
        )
    }
}
