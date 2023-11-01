use objects::make_get_object_info_fn;
use serde_json::json;

use crate::objects::list::get_available_objects;

mod config;
pub mod objects;
#[cfg(test)]
mod packing;

#[test]
fn gen_save_spritesheet() {
    use packing::make_spritesheet;
    use std::fs;

    let (img, data) = make_spritesheet();

    img.save("../src/gd/spritesheet.png").unwrap();
    fs::write(
        "../src/gd/spritesheet.json",
        serde_json::to_string(&json!(data)).unwrap(),
    )
    .unwrap();
}

#[test]
fn save_object_info() {
    use std::fs;

    fs::write(
        "../src/gd/objects.json",
        serde_json::to_string(&json!(get_available_objects())).unwrap(),
    )
    .unwrap();
}

#[test]
fn make_wasm_lib_utilgen() {
    use std::fs;
    let contents = format!(
        "
use the_nexus::objects::ObjectInfo;

{}

    ",
        make_get_object_info_fn()
    );
    fs::write("../wasm-lib/src/utilgen.rs", contents).unwrap();
}
