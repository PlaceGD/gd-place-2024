#[cfg(test)]
use packing::SpritesheetData;

use crate::colors::get_available_colors;

pub mod colors;
mod config;
pub mod objects;
pub mod packing;

#[test]
fn generate_shid() {
    use crate::objects::list::get_available_objects;
    use packing::make_spritesheet;
    use serde_json::json;
    use std::fs;

    let (img, data) = make_spritesheet();

    img.save("../src/gd/spritesheet.png").unwrap();
    fs::write(
        "../src/gd/spritesheet.json",
        serde_json::to_string(&json!(data)).unwrap(),
    )
    .unwrap();

    fs::write(
        "../src/gd/objects.json",
        serde_json::to_string(&json!(get_available_objects())).unwrap(),
    )
    .unwrap();
    fs::write(
        "../src/gd/colors.json",
        serde_json::to_string(&get_available_colors()).unwrap(),
    )
    .unwrap();

    fs::write("../wasm-lib/src/utilgen.rs", make_wasm_lib_utilgen(&data)).unwrap();
}

#[cfg(test)]
fn make_wasm_lib_utilgen(sheet_data: &SpritesheetData) -> String {
    use crate::{
        objects::make_get_object_info_fn,
        packing::{make_get_detail_sprite_fn, make_get_main_sprite_fn},
    };

    format!(
        "
use the_nexus::objects::{{ObjectCategory::*, ObjectInfo}};
use the_nexus::packing::SpriteInfo;

{}

{}

{}

    ",
        make_get_object_info_fn(),
        make_get_main_sprite_fn(sheet_data),
        make_get_detail_sprite_fn(sheet_data),
    )
}
