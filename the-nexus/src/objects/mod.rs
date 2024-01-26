use itertools::Itertools;

use crate::objects::list::get_available_objects;

pub mod list;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ObjectCategory {
    Blocks,
    Outlines,
    Spikes,
    OrbsAndGlorbs,
    Pixel,
    Deco,
    Saws,
}
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectInfo {
    pub place_offset_x: f32,
    pub place_offset_y: f32,
    pub tintable: bool,
    pub solid: bool,
    pub builtin_scale: f32,
    pub category: ObjectCategory,
}

pub(crate) fn make_get_object_info_fn() -> String {
    let mut ongy = [ObjectInfo {
        place_offset_x: 0.0,
        place_offset_y: 0.0,
        tintable: true,
        solid: false,
        builtin_scale: 1.0,
        category: ObjectCategory::Blocks,
    }; 4600];
    for &(k, v) in get_available_objects() {
        ongy[k as usize] = v;
    }

    format!(
        "
pub const OBJECT_INFO: [ObjectInfo; 4600] = {ongy:?};
    "
    )
}
