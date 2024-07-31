use gen::{colors::get_available_colors, objects::make_get_object_info_fn};

mod gen;
mod objects;
mod sprites;
mod util;

pub use objects::{list::AVAILABLE_OBJECTS, HitboxType, ObjectCategory, ObjectInfo};
pub use sprites::SpriteInfo;

fn make_wasm_lib_utilgen(sheet_data: &gen::sprites::SpritesheetData) -> String {
    use crate::gen::sprites::{make_get_detail_sprite_fn, make_get_main_sprite_fn};

    format!(
        "
use the_nexus::{{ObjectCategory::*, HitboxType::*, ObjectInfo, SpriteInfo}};

{}

{}

{}

    ",
        make_get_object_info_fn(),
        make_get_main_sprite_fn(sheet_data),
        make_get_detail_sprite_fn(sheet_data),
    )
}

#[test]
fn generate_shid() {
    use std::collections::HashMap;

    use crate::gen::sprites::color_bleed;
    use crate::objects::list::AVAILABLE_OBJECTS;
    use gen::sprites::make_spritesheet;
    use itertools::Itertools;
    use serde_json::json;
    use std::fs;

    let (mut img, data) = make_spritesheet();
    color_bleed(&mut img);

    img.save("../public/textures/spritesheet.png").unwrap();
    fs::write(
        "../shared-lib/src/gd/spritesheet.json",
        serde_json::to_string(&json!(data)).unwrap(),
    )
    .unwrap();

    fs::write(
        "../shared-lib/src/gd/objects.json",
        serde_json::to_string(&json!(AVAILABLE_OBJECTS
            .iter()
            .copied()
            .collect::<HashMap<_, _>>()))
        .unwrap(),
    )
    .unwrap();
    fs::write(
        "../shared-lib/src/gd/object_order.json",
        serde_json::to_string(&json!(AVAILABLE_OBJECTS.iter().map(|v| v.0).collect_vec())).unwrap(),
    )
    .unwrap();
    fs::write(
        "../shared-lib/src/gd/colors.json",
        serde_json::to_string(&get_available_colors()).unwrap(),
    )
    .unwrap();

    fs::write("../wasm-lib/src/utilgen.rs", make_wasm_lib_utilgen(&data)).unwrap();
}
