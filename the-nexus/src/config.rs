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
