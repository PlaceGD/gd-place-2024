use gen::{
    colors::get_available_colors, countdown_digits::make_get_countdown_digits_fn,
    objects::make_get_object_info_fn,
};
use objects::sfx::SFX_TRIGGER_SOUNDS;
use rust_shared::gd::special_ids;
use serde_json::json;

mod gen;
mod objects;

fn make_wasm_lib_utilgen(sheet_data: &gen::sprites::SpritesheetData) -> String {
    use crate::gen::sprites::{
        make_get_detail_sprite_fn, make_get_main_sprite_fn, make_get_sfx_icon_sprite_fn,
    };

    format!(
        "
use rust_shared::{{gd::{{ObjectCategory::*, HitboxType::*, ObjectInfo}}, sprite::SpriteInfo}};
// use the_nexus::{{ObjectCategory::*, HitboxType::*, ObjectInfo, SpriteInfo}};

{}

{}

{}

{}

    ",
        make_get_object_info_fn(),
        make_get_main_sprite_fn(sheet_data),
        make_get_detail_sprite_fn(sheet_data),
        make_get_sfx_icon_sprite_fn(sheet_data),
    )
}

#[test]
fn generate_shid() {
    generate_shide(true)
}

#[test]
fn generate_shid_no_sheet() {
    generate_shide(false)
}

fn generate_shide(sheet: bool) {
    use std::collections::HashMap;

    use crate::gen::sprites::color_bleed;
    use crate::objects::list::AVAILABLE_OBJECTS;
    use gen::sprites::make_spritesheet;
    use itertools::Itertools;
    use std::fs;
    if sheet {
        let (mut img, data) = make_spritesheet();
        color_bleed(&mut img);

        img.save("../../public/assets/spritesheet.png").unwrap();
        fs::write(
            "../../shared-lib/src/gd/spritesheet.json",
            serde_json::to_string(&json!(data)).unwrap(),
        )
        .unwrap();
        fs::write("../wasm-lib/src/utilgen.rs", make_wasm_lib_utilgen(&data)).unwrap();
    }
    fs::write(
        "../wasm-lib/src/countdown_digits",
        make_get_countdown_digits_fn(),
    )
    .unwrap();

    fs::write(
        "../../shared-lib/src/gd/objects.json",
        serde_json::to_string(&json!(AVAILABLE_OBJECTS
            .iter()
            .copied()
            .collect::<HashMap<_, _>>()))
        .unwrap(),
    )
    .unwrap();
    fs::write(
        "../../shared-lib/src/gd/object_order.json",
        serde_json::to_string(&json!(AVAILABLE_OBJECTS.iter().map(|v| v.0).collect_vec())).unwrap(),
    )
    .unwrap();
    fs::write(
        "../../shared-lib/src/gd/colors.json",
        serde_json::to_string(&get_available_colors()).unwrap(),
    )
    .unwrap();
    fs::write(
        "../../shared-lib/src/nexusgen.ts",
        format!(
            "
export const BG_TRIGGER: number = {};
export const GROUND_TRIGGER: number = {};
export const GROUND_2_TRIGGER: number = {};
export const ARROW_TRIGGER: number = {};
export const SFX_TRIGGER: number = {};
export const SONG_TRIGGER: number = {};

export const TRIGGERS: number[] = {:?};
export const COLOR_TRIGGERS: number[] = {:?};

export const SFX_TRIGGER_SOUNDS: string[] = {:?};
    ",
            special_ids::BG_TRIGGER,
            special_ids::GROUND_TRIGGER,
            special_ids::GROUND_2_TRIGGER,
            special_ids::ARROW_TRIGGER,
            special_ids::SFX_TRIGGER,
            special_ids::SONG_TRIGGER,
            special_ids::TRIGGERS,
            special_ids::COLOR_TRIGGERS,
            SFX_TRIGGER_SOUNDS,
        ),
    )
    .unwrap();
}
