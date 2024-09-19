use std::{collections::HashMap, io::Read};

use base64::Engine;
use flate2::read::GzDecoder;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use rust_shared::gd::{object::GDColor, special_ids, HitboxType, ObjectCategory, ObjectInfo};

use crate::objects::levelstring::ObjectMap;

use super::levelstring::parse_obj;

pub static AVAILABLE_OBJECTS: Lazy<Box<[(u16, ObjectInfo)]>> = Lazy::new(|| {
    let objects = {
        let lvl = include_str!("placeobjs.gmd");
        parse_gmd_file(lvl).objects
    };
    let object_hitbox_types = include_str!("object_types.txt")
        .lines()
        .map(|v| {
            v.split(':')
                .map(|v| v.parse::<u16>().unwrap())
                .tuple_windows()
                .next()
                .unwrap()
        })
        .map(|(a, b)| (a, game_object_type_to_hitbox_type(b)))
        .collect::<HashMap<_, _>>();

    let categories: Vec<((f32, f32), ObjectCategory)> = objects
        .iter()
        .filter(|v| v[&1] == "914")
        .map(|v| v[&2].parse::<f32>().unwrap())
        .chain([1000000.0])
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .tuple_windows()
        .zip([
            ObjectCategory::OrbsAndGlorbs,
            ObjectCategory::Pixel,
            ObjectCategory::Blocks,
            ObjectCategory::Outlines,
            ObjectCategory::Spikes,
            ObjectCategory::Deco,
            ObjectCategory::Saws,
        ])
        .collect_vec();
    println!("{:?}", categories);

    let center_xy = |x: f32, y: f32| {
        (
            (x / 30.0).floor() * 30.0 + 15.0,
            (y / 30.0).floor() * 30.0 + 15.0,
        )
    };

    let sort_key = |v: &ObjectMap| {
        let (cx, cy) = center_xy(v[&2].parse::<f32>().unwrap(), v[&3].parse::<f32>().unwrap());
        cx - 100000.0 * cy
    };

    let objects = objects
        .iter()
        .filter(|v| v[&1] != "914")
        .sorted_by(|a, b| sort_key(a).partial_cmp(&sort_key(b)).unwrap())
        .map(|v| {
            let id = v[&1].parse::<u16>().unwrap();

            let x = v[&2].parse::<f32>().unwrap();
            let y = v[&3].parse::<f32>().unwrap();
            let category = categories
                .iter()
                .find(|c| x >= c.0 .0 && x < c.0 .1)
                .unwrap()
                .1;
            let scale = v.get(&32).map(|v| v.parse::<f32>().unwrap()).unwrap_or(1.0);
            let scale_x = v
                .get(&128)
                .map(|v| v.parse::<f32>().unwrap())
                .unwrap_or(1.0);
            let scale_y = v
                .get(&129)
                .map(|v| v.parse::<f32>().unwrap())
                .unwrap_or(1.0);

            let (cx, cy) = center_xy(x, y);

            (
                id,
                ObjectInfo {
                    place_offset_x: x - cx,
                    place_offset_y: y - cy,
                    hitbox_type: object_hitbox_types
                        .get(&id)
                        .copied()
                        .unwrap_or(HitboxType::NoHitbox),
                    builtin_scale_x: scale * scale_x,
                    builtin_scale_y: scale * scale_y,
                    category,
                },
            )
        })
        .chain(
            [
                special_ids::BG_TRIGGER,
                special_ids::GROUND_TRIGGER,
                special_ids::GROUND_2_TRIGGER,
                special_ids::ARROW_TRIGGER,
                special_ids::SFX_TRIGGER,
                special_ids::SONG_TRIGGER,
            ]
            .map(|v| {
                (
                    v,
                    ObjectInfo {
                        place_offset_x: 0.0,
                        place_offset_y: 0.0,
                        hitbox_type: HitboxType::Special,
                        builtin_scale_x: 0.5,
                        builtin_scale_y: 0.5,
                        category: ObjectCategory::Triggers,
                    },
                )
            }),
        )
        .collect_vec()
        .into_boxed_slice();

    objects
});

pub struct LevelParseResult {
    pub objects: Vec<ObjectMap>,
    pub colors: HashMap<u16, GDColor>,
}

// #[derive(Debug, Clone)]
// pub struct GDColor {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
//     pub opacity: u8,
//     pub blending: bool,
// }
// impl GDColor {
//     pub(crate) fn default() -> GDColor {
//         GDColor {
//             r: 255,
//             g: 255,
//             b: 255,
//             opacity: 255,
//             blending: false,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct GDObject {
//     pub id: u16,
//     pub x: f32,
//     pub y: f32,

//     pub ix: f32,
//     pub iy: f32,
//     pub jx: f32,
//     pub jy: f32,

//     pub z_layer: i8,
//     pub z_order: i8,
//     pub main_color: GDColor,
//     pub detail_color: GDColor,
// }

pub fn parse_gmd_file(lvl: &str) -> LevelParseResult {
    let r = Regex::new(r##"H4sIAAA[A-Za-z0-9-_=]+"##).unwrap();

    let lvl = r.find(lvl).unwrap().as_str().as_bytes();

    let decoded = base64::engine::general_purpose::URL_SAFE
        .decode(lvl)
        .unwrap();

    let mut d = GzDecoder::new(&decoded[..]);
    let mut lvl = String::new();
    d.read_to_string(&mut lvl).unwrap();

    let mut iter = lvl.split(';');

    let colors = iter
        .next()
        .unwrap()
        .split(',')
        .tuples()
        .find(|(k, _)| *k == "kS38")
        .unwrap()
        .1
        .split('|')
        .map(|c| {
            let map = c
                .split('_')
                .tuples()
                .map(|(k, v)| (k.parse().unwrap(), v))
                .collect::<HashMap<u16, &str>>();
            (
                map.get(&6).unwrap_or(&"1").parse().unwrap(),
                GDColor {
                    r: map.get(&1).unwrap_or(&"0").parse().unwrap(),
                    g: map.get(&2).unwrap_or(&"0").parse().unwrap(),
                    b: map.get(&3).unwrap_or(&"0").parse().unwrap(),
                    opacity: (map.get(&7).unwrap_or(&"1.0").parse::<f32>().unwrap() * 255.0) as u8,
                    blending: map.get(&5) == Some(&"1"),
                },
            )
        })
        .collect();

    let objects = iter.filter(|v| !v.is_empty()).map(parse_obj).collect_vec();

    LevelParseResult { objects, colors }
}

fn game_object_type_to_hitbox_type(typ: u16) -> HitboxType {
    match typ {
        // Solid
        0 => HitboxType::Solid,
        // Hazard
        2 => HitboxType::Hazard,
        // InverseGravityPortal
        3 => HitboxType::Special,
        // NormalGravityPortal
        4 => HitboxType::Special,
        // ShipPortal
        5 => HitboxType::Special,
        // CubePortal
        6 => HitboxType::Special,
        // Decoration
        7 => HitboxType::NoHitbox,
        // YellowJumpPad
        8 => HitboxType::Special,
        // PinkJumpPad
        9 => HitboxType::Special,
        // GravityPad
        10 => HitboxType::Special,
        // YellowJumpRing
        11 => HitboxType::Special,
        // PinkJumpRing
        12 => HitboxType::Special,
        // GravityRing
        13 => HitboxType::Special,
        // InverseMirrorPortal
        14 => HitboxType::Special,
        // NormalMirrorPortal
        15 => HitboxType::Special,
        // BallPortal
        16 => HitboxType::Special,
        // RegularSizePortal
        17 => HitboxType::Special,
        // MiniSizePortal
        18 => HitboxType::Special,
        // UfoPortal
        19 => HitboxType::Special,
        // Modifier
        20 => HitboxType::Special,
        // Breakable
        21 => HitboxType::Solid,
        // SecretCoin
        22 => HitboxType::Special,
        // DualPortal
        23 => HitboxType::Special,
        // SoloPortal
        24 => HitboxType::Special,
        // Slope
        25 => HitboxType::Solid,
        // WavePortal
        26 => HitboxType::Special,
        // RobotPortal
        27 => HitboxType::Special,
        // TeleportPortal
        28 => HitboxType::Special,
        // GreenRing
        29 => HitboxType::Special,
        // Collectible
        30 => HitboxType::Special,
        // UserCoin
        31 => HitboxType::Special,
        // DropRing
        32 => HitboxType::Special,
        // SpiderPortal
        33 => HitboxType::Special,
        // RedJumpPad
        34 => HitboxType::Special,
        // RedJumpRing
        35 => HitboxType::Special,
        // CustomRing
        36 => HitboxType::Special,
        // DashRing
        37 => HitboxType::Special,
        // GravityDashRing
        38 => HitboxType::Special,
        // CollisionObject
        39 => HitboxType::Special,
        // Special
        40 => HitboxType::Special,
        // SwingPortal
        41 => HitboxType::Special,
        // GravityTogglePortal
        42 => HitboxType::Special,
        // SpiderOrb
        43 => HitboxType::Special,
        // SpiderPad
        44 => HitboxType::Special,
        // TeleportOrb
        46 => HitboxType::Special,
        // AnimatedHazard
        47 => HitboxType::Hazard,
        _ => HitboxType::NoHitbox,
    }
}
