// pub mod cool_level;
pub mod layer;
pub mod level;
pub mod object;

pub mod special_ids {
    pub const BG_TRIGGER: u16 = 4550;
    pub const GROUND_TRIGGER: u16 = 4551;
    pub const GROUND_2_TRIGGER: u16 = 4552;

    pub const ARROW_TRIGGER: u16 = 4553;

    pub const SFX_TRIGGER: u16 = 4554;
    pub const SONG_TRIGGER: u16 = 4555;

    pub const TRIGGERS: &[u16] = &[
        BG_TRIGGER,
        GROUND_TRIGGER,
        GROUND_2_TRIGGER,
        ARROW_TRIGGER,
        SFX_TRIGGER,
        SONG_TRIGGER,
    ];
    pub const COLOR_TRIGGERS: &[u16] = &[BG_TRIGGER, GROUND_TRIGGER, GROUND_2_TRIGGER];
}

// make sure to modify all of these in sharedlib gd.ts too!!!!
// make sure to modify all of these in sharedlib gd.ts too!!!!
// make sure to modify all of these in sharedlib gd.ts too!!!!
// make sure to modify all of these in sharedlib gd.ts too!!!!
// make sure to modify all of these in sharedlib gd.ts too!!!!
// make sure to modify all of these in sharedlib gd.ts too!!!!

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ObjectCategory {
    Blocks,
    Outlines,
    Spikes,
    OrbsAndGlorbs,
    Pixel,
    Deco,
    Saws,
    Triggers,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HitboxType {
    NoHitbox,
    Solid,
    Hazard,
    Special,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ObjectSheet {
    GJParticleSheet,
    PixelSheet01,
    GJGameSheet02,
    FireSheet01,
    GJGameSheet,
}
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectInfo {
    pub place_offset_x: f32,
    pub place_offset_y: f32,
    pub hitbox_type: HitboxType,
    pub builtin_scale_x: f32,
    pub builtin_scale_y: f32,
    pub category: ObjectCategory,
    pub sheet: ObjectSheet,
}
