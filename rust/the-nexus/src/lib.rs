use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use gen::{
    colors::get_available_colors,
    countdown_digits::make_get_countdown_digits_fn,
    objects::{make_get_object_info_fn, make_get_object_main_over_detail},
    sprites::make_get_song_icon_sprite_fn,
};
use objects::{sfx::SFX_TRIGGER_SOUNDS, song::SONG_TRIGGER_SONGS};
use rust_shared::{
    countdown::generate_set_switches,
    gd::{
        level::{
            CHUNK_SIZE_BLOCKS, END_POS_X, END_POS_Y, END_RADIUS, LEVEL_HEIGHT_BLOCKS,
            LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_BLOCKS, LEVEL_WIDTH_UNITS,
        },
        object::GDObject,
        special_ids,
    },
};
use serde_json::json;

mod gen;
mod objects;

fn make_wasm_lib_utilgen(sheet_data: &gen::sprites::SpritesheetData) -> String {
    use crate::gen::sprites::{
        make_get_detail_sprite_fn, make_get_main_sprite_fn, make_get_sfx_icon_sprite_fn,
    };

    format!(
        "
use rust_shared::{{gd::{{ObjectCategory::*, HitboxType::*, ObjectSheet::*, ObjectInfo}}, sprite::SpriteInfo}};
// use the_nexus::{{ObjectCategory::*, HitboxType::*, ObjectInfo, SpriteInfo}};

{}

{}

{}

{}

{}

{}

{}

    ",
        make_get_object_info_fn(),
        make_get_main_sprite_fn(sheet_data),
        make_get_detail_sprite_fn(sheet_data),
        make_get_sfx_icon_sprite_fn(sheet_data),
        make_get_song_icon_sprite_fn(sheet_data),
        make_get_set_switches_fn(),
        make_get_object_main_over_detail(),
    )
}

#[test]
fn generate_shid() {
    generate_shide(true)
}

#[test]
fn generate_shimabid_no_sheet() {
    generate_shide(false)
}

fn generate_shide(sheet: bool) {
    use std::collections::HashMap;

    use crate::gen::sprites::color_bleed;
    use crate::objects::list::AVAILABLE_OBJECTS;
    use gen::sprites::make_spritesheet;
    use itertools::Itertools;
    use std::fs;
    eprintln!("GLABOBOBOB");
    if !PathBuf::from("../../src/assets").exists() {
        std::fs::create_dir("../../src/assets").unwrap();
    }
    if !PathBuf::from("../../shared-lib/src/gd").exists() {
        std::fs::create_dir("../../shared-lib/src/gd").unwrap();
    }

    if sheet {
        let (mut img, data) = make_spritesheet();
        color_bleed(&mut img);

        img.save("../../src/assets/spritesheet.png").unwrap();
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
export const SONG_TRIGGER_SONGS: string[] = {:?};

export const CHUNK_SIZE_BLOCKS = {};
export const CHUNK_SIZE_UNITS = CHUNK_SIZE_BLOCKS * 30;

export const LEVEL_WIDTH_BLOCKS = {};
export const LEVEL_HEIGHT_BLOCKS = {};
export const LEVEL_WIDTH_UNITS = {};
export const LEVEL_HEIGHT_UNITS = {};

export const END_POS_X = {};
export const END_POS_Y = {};
export const END_RADIUS = {};
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
            SONG_TRIGGER_SONGS,
            CHUNK_SIZE_BLOCKS,
            LEVEL_WIDTH_BLOCKS,
            LEVEL_HEIGHT_BLOCKS,
            LEVEL_WIDTH_UNITS,
            LEVEL_HEIGHT_UNITS,
            END_POS_X,
            END_POS_Y,
            END_RADIUS,
        ),
    )
    .unwrap();
}

fn make_get_set_switches_fn() -> String {
    let switches = generate_set_switches(512);

    format!("pub const SET_SWITCHES: &[[usize; 4]] = &{:?};", switches)
}

mod dbconvert {
    use std::fs;

    use num::{BigInt, BigUint};
    use serde_json::json;

    use crate::gen::countdown_digits::get_transform;

    // pub fn convert_chunk(chunkkey: &str) -> &str {
    //     let a = chunkkey
    //         .split(',')
    //         .map(|v| v.parse::<i32>().unwrap())
    //         .collect::<Vec<_>>();

    //     let x = a[0];
    //     let y = a[1];
    // }

    pub(crate) fn convert_old_obj_to_new(old_obj: &str) -> String {
        let values = old_obj.split(';').collect::<Vec<_>>();
        let id = values[0].parse::<u16>().unwrap();
        let x = values[1].parse::<f32>().unwrap();
        let y = values[2].parse::<f32>().unwrap();
        let rotation = values[3].parse::<f32>().unwrap().to_radians();
        let flip = values[4] == "1";
        let scale = values[5].parse::<f32>().unwrap();
        let z_order = values[6].parse::<i8>().unwrap();
        // hex
        let main_color = u32::from_str_radix(values[7], 16).unwrap();
        let main_blending = values[8] == "1";
        let main_opacity = values[9].parse::<f32>().unwrap();
        let detail_color = u32::from_str_radix(values[10], 16).unwrap();
        let detail_blending = values[11] == "1";
        let detail_opacity = values[12].parse::<f32>().unwrap();

        let [ix, iy, jx, jy] = get_transform(
            if flip { -scale } else { scale },
            -rotation,
            scale,
            -rotation + std::f32::consts::PI / 2.0,
        );

        let mut z_layer = 1;

        if main_blending || detail_blending {
            if z_order > 45 {
                z_layer = 3;
            } else {
                z_layer = -1;
            }
        }

        let main_color = rust_shared::gd::object::GDColor {
            r: (main_color >> 16) as u8,
            g: (main_color >> 8) as u8,
            b: main_color as u8,
            opacity: (main_opacity * 255.0) as u8,
            blending: main_blending,
        };

        let detail_color = rust_shared::gd::object::GDColor {
            r: (detail_color >> 16) as u8,
            g: (detail_color >> 8) as u8,
            b: detail_color as u8,
            opacity: (detail_opacity * 255.0) as u8,
            blending: detail_blending,
        };

        let z_layer = rust_shared::gd::layer::ZLayer::from_gd_num(z_layer);

        let [x_scale_exp, x_angle, y_scale_exp, y_angle] =
            convert_from_opt_transform(ix, iy, jx, jy);

        let obj = GDObjectOptButNotReally {
            id,
            x,
            y,
            x_scale_exp,
            x_angle,
            y_scale_exp,
            y_angle,
            z_layer,
            z_order,
            main_color,
            detail_color,
        };

        let bytes: [u8; std::mem::size_of::<GDObjectOptButNotReally>()] =
            unsafe { std::mem::transmute(obj) };

        encode_string(&bytes, 126)
    }
    #[repr(C, packed)]
    pub struct GDObjectOptButNotReally {
        pub id: u16,
        pub x: f32,
        pub y: f32,

        pub x_scale_exp: i8,
        pub x_angle: i8,

        pub y_scale_exp: i8,
        pub y_angle: i8,

        pub z_layer: rust_shared::gd::layer::ZLayer,
        pub z_order: i8,
        pub main_color: rust_shared::gd::object::GDColor,
        pub detail_color: rust_shared::gd::object::GDColor,
    }

    // chatgpt lol
    pub fn convert_from_opt_transform(ix: f32, iy: f32, jx: f32, jy: f32) -> [i8; 4] {
        // Compute scales from the vector magnitudes
        let x_scale = (ix.powi(2) + iy.powi(2)).sqrt();
        let y_scale = (jx.powi(2) + jy.powi(2)).sqrt();

        // Compute angles in radians from the vector components
        let x_angle_rad = iy.atan2(ix);
        let y_angle_rad = jy.atan2(jx);

        // Convert scales back to scale exponents
        let x_scale_exp = (12.0 * x_scale.log2()).round() as i8;
        let y_scale_exp = (12.0 * y_scale.log2()).round() as i8;

        // Convert angles from radians to degrees, then to the original angle units
        let x_angle_deg = x_angle_rad.to_degrees();
        let y_angle_deg = y_angle_rad.to_degrees();

        let x_angle = (x_angle_deg / 5.0).round() as i8;
        let y_angle = (y_angle_deg / 5.0).round() as i8;

        [x_scale_exp, x_angle, y_scale_exp, y_angle]
    }

    #[test]
    pub(crate) fn convert_old_db_to_new() {
        let data = fs::read_to_string("C:/p/geometrydash-place-default-rtdb-export.json");
        let old_db: serde_json::Value = serde_json::from_str(&data.unwrap()).unwrap();

        let new_objects = old_db["chunks"]
            .as_object()
            .unwrap()
            .into_iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    serde_json::Value::Object(
                        v.as_object()
                            .unwrap()
                            .into_iter()
                            .map(|(k, v)| {
                                (
                                    k.clone(),
                                    serde_json::Value::String(convert_old_obj_to_new(
                                        v.as_str().unwrap(),
                                    )),
                                )
                            })
                            .collect::<serde_json::Map<_, _>>(),
                    ),
                )
            })
            .collect::<serde_json::Map<_, _>>();

        let new_username = old_db["userName"]
            .as_object()
            .unwrap()
            .into_iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    json!({"uid": v["uid"].as_str().unwrap(), "displayColor": "white"}),
                )
            })
            .collect::<serde_json::Map<_, _>>();

        let user_details = old_db["userData"].as_object().unwrap().clone();

        let new_user_placed = old_db["userPlaced"].as_object().unwrap().clone();

        let new_object_count = old_db["objectCount"].as_object().unwrap().clone();

        let new_db = json!({
            "objects": new_objects,
            "userName": new_username,
            "userDetails": user_details,
            "userPlaced": new_user_placed,
            "objectCount": new_object_count,
        });

        fs::write(
            "C:/p/new_db.json",
            serde_json::to_string_pretty(&new_db).unwrap(),
        )
        .unwrap();
    }

    pub(crate) fn baseconvert(digits: &[u8], from_base: u32, to_base: u32) -> Vec<u8> {
        let big_from_base = BigUint::from(from_base);
        let big_to_base = BigUint::from(to_base);
        let mut big_sum = BigUint::from(0u8);

        let mut zeroes = 0;
        for i in 0..digits.len() {
            if digits[i] == 0 {
                zeroes += 1;
            } else {
                break;
            }
        }

        for i in 0..digits.len() {
            let p = digits.len() - 1 - i;
            big_sum += digits[i] as u32 * big_from_base.pow(p as u32);
        }

        use num::NumCast;

        let mut ret: Vec<u8> = vec![];
        while big_sum > BigUint::from(0u8) {
            let digit = big_sum.clone() % &big_to_base;
            ret.push(NumCast::from(digit).unwrap());
            big_sum /= &big_to_base;
        }
        ret.extend(std::iter::repeat(0).take(zeroes));
        ret.reverse();
        ret
    }

    pub(crate) fn encode(data: &[u8], base: u32) -> Vec<u8> {
        baseconvert(data, 256, base)
    }

    pub(crate) fn encode_string(data: &[u8], base: u32) -> String {
        let utf8 = encode(data, base);
        String::from_utf8(utf8).unwrap()
    }
}
