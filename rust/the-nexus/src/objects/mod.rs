pub mod levelstring;
pub mod list;
pub mod sfx;

// make sure to modify ObjectCategory and ObjectHitbox and ObjectInfo in sharedlib gd.ts too!!!!
// make sure to modify ObjectCategory and ObjectHitbox and ObjectInfo in sharedlib gd.ts too!!!!
// make sure to modify ObjectCategory and ObjectHitbox and ObjectInfo in sharedlib gd.ts too!!!!
// make sure to modify ObjectCategory and ObjectHitbox and ObjectInfo in sharedlib gd.ts too!!!!
// make sure to modify ObjectCategory and ObjectHitbox and ObjectInfo in sharedlib gd.ts too!!!!
// make sure to modify ObjectCategory and ObjectHitbox and ObjectInfo in sharedlib gd.ts too!!!!

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
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectInfo {
    pub place_offset_x: f32,
    pub place_offset_y: f32,
    pub hitbox_type: HitboxType,
    pub builtin_scale_x: f32,
    pub builtin_scale_y: f32,
    pub category: ObjectCategory,
}
