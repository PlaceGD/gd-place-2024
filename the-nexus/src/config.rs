#[cfg(test)]
use texture_packer::TexturePackerConfig;
#[cfg(test)]
pub const PACKER_CONFIG: TexturePackerConfig = TexturePackerConfig {
    max_width: 4096,
    max_height: 4096,
    allow_rotation: true,
    border_padding: 0,
    texture_padding: 0,
    texture_extrusion: 1,
    trim: true,
    texture_outlines: false,
};
