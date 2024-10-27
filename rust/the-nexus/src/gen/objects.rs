use rust_shared::gd::{HitboxType, ObjectCategory, ObjectInfo, ObjectSheet};

use crate::objects::list::AVAILABLE_OBJECTS;

pub fn make_get_object_info_fn() -> String {
    let mut ongy = [ObjectInfo {
        place_offset_x: 0.0,
        place_offset_y: 0.0,

        category: ObjectCategory::Blocks,
        hitbox_type: HitboxType::NoHitbox,
        builtin_scale_x: 1.0,
        builtin_scale_y: 1.0,
        sheet: ObjectSheet::GJGameSheet,
    }; 4600];
    for &(k, v) in AVAILABLE_OBJECTS.iter() {
        ongy[k as usize] = v;
    }

    format!(
        "
pub const OBJECT_INFO: [ObjectInfo; 4600] = {ongy:?};
    "
    )
}

pub fn make_get_object_main_over_detail() -> String {
    let mut ongy = Vec::new();
    for id in include_str!("../objects/main_over_detail_objects.txt")
        .lines()
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<u16>().unwrap())
    {
        ongy.push(id);
    }

    format!(
        "
pub const OBJECT_MAIN_OVER_DETAIL_IDS: &[u16] = &{ongy:?};
    "
    )
}
