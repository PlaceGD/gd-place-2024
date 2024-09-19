use crate::objects::{list::AVAILABLE_OBJECTS, HitboxType, ObjectCategory, ObjectInfo};

pub fn make_get_object_info_fn() -> String {
    let mut ongy = [ObjectInfo {
        place_offset_x: 0.0,
        place_offset_y: 0.0,

        category: ObjectCategory::Blocks,
        hitbox_type: HitboxType::NoHitbox,
        builtin_scale_x: 1.0,
        builtin_scale_y: 1.0,
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
