use texture_packer::TexturePackerConfig;

pub const PACKER_CONFIG: TexturePackerConfig = TexturePackerConfig {
    max_width: 4096,
    max_height: 4096,
    allow_rotation: false,
    border_padding: 0,
    texture_padding: 5,
    texture_extrusion: 2,
    trim: true,
    texture_outlines: false,
};

// 0-360
pub fn available_hues() -> impl IntoIterator<Item = u16> {
    [0, 30, 60, 80, 120, 160, 180, 210, 240, 270, 300, 330]
}

// brightness
pub const PICKER_ROWS: u32 = 4;

// saturation
pub const PICKER_COLUMNS: u32 = 4;
