use itertools::Itertools;

use crate::objects::list::get_available_objects;

pub mod list;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ObjectCategory {
    Blocks,
    Outlines,
    Slopes,
    Spikes,
    Utilities,
    GroundDeco,
    Deco,
    Pulsing,
    Saws,
}
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectInfo {
    pub place_offset_x: f32,
    pub place_offset_y: f32,
    pub tintable: bool,
    pub solid: bool,
    pub category: ObjectCategory,
}

pub(crate) fn make_get_object_info_fn() -> String {
    format!(
        "
pub fn get_object_info(id: u32) -> Option<ObjectInfo> {{
    Some(match id {{
        {},
        _ => return None,
    }})
}}
    ",
        get_available_objects()
            .iter()
            .map(|(id, info)| { format!("{id} => {info:#?}") })
            .join(",")
    )
}
