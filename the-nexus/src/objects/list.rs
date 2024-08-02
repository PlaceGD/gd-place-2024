use std::{collections::HashMap, io::Read};

use base64::Engine;
use flate2::read::GzDecoder;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use super::{levelstring::parse_obj, ObjectInfo};

use crate::objects::{levelstring::ObjectMap, HitboxType, ObjectCategory};

pub mod special_ids {
    pub const BG_TRIGGER: u32 = 4550;
    pub const GROUND_TRIGGER: u32 = 4551;
    pub const GROUND_2_TRIGGER: u32 = 4552;
}

pub static AVAILABLE_OBJECTS: Lazy<Box<[(u32, ObjectInfo)]>> = Lazy::new(|| {
    let objects = {
        let lvl = include_str!("placeobjs.gmd");
        let r = Regex::new(r##"H4sIAAA[A-Za-z0-9-_=]+"##).unwrap();

        let lvl = r.find(lvl).unwrap().as_str().as_bytes();

        let decoded = base64::engine::general_purpose::URL_SAFE
            .decode(lvl)
            .unwrap();

        let mut d = GzDecoder::new(&decoded[..]);
        let mut lvl = String::new();
        d.read_to_string(&mut lvl).unwrap();

        lvl.split(';')
            .filter(|v| !v.is_empty())
            .skip(1)
            .map(parse_obj)
            .collect_vec()
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

    let sort_key = |v: &ObjectMap| {
        let x = v[&2].parse::<f32>().unwrap();
        let y = v[&3].parse::<f32>().unwrap();
        x - 100000.0 * y
    };

    let mut objects = objects
        .iter()
        .filter(|v| v[&1] != "914")
        .sorted_by(|a, b| sort_key(a).partial_cmp(&sort_key(b)).unwrap())
        .map(|v| {
            let id = v[&1].parse::<u16>().unwrap();

            let x = v[&2].parse::<f32>().unwrap();
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
            (
                v[&1].parse::<u32>().unwrap(),
                ObjectInfo {
                    place_offset_x: 0.0,
                    place_offset_y: 0.0,
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
