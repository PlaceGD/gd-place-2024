use core::fmt;
use std::{
    array,
    collections::{HashMap, HashSet},
    io::Cursor,
};

use binrw::BinWrite;
use itertools::Itertools;
use rust_shared::{
    console_log,
    countdown::{
        get_countdown_sets, get_creator_name, CountdownDigitSets, DigitObjects, DigitSet,
        DIGIT_HEIGHT, DIGIT_SETS, DIGIT_WIDTH,
    },
    gd::{
        layer::ZLayer,
        object::{GDColor, GDObject},
    },
};

use crate::objects::list::{parse_gmd_file, AVAILABLE_OBJECTS, OBJECT_DEFAULT_Z};

pub fn make_get_countdown_digits_fn() -> Vec<u8> {
    let (files, sets) = get_countdown_sets(parse_gmd_file);

    let x_offset = 8.0 * 30.0;
    let y_offset = 15.0 * 30.0;

    let h_radius = DIGIT_WIDTH / 2.0;
    let v_radius = DIGIT_HEIGHT / 2.0;

    let all_avaliable_ids = AVAILABLE_OBJECTS
        .iter()
        .map(|(k, _)| *k)
        .collect::<HashSet<u16>>();

    let mut missing = HashMap::new();

    let mut set_i = 0;

    let sets: [[DigitObjects; 10]; DIGIT_SETS] = sets.map(|set_ptr| {
        let parsed = &files[set_ptr.file];
        let digit_set = set_ptr.set;
        set_i += 1;
        array::from_fn(|digit| {
            let x = x_offset + digit as f32 * DIGIT_WIDTH;
            let y = y_offset + digit_set as f32 * DIGIT_HEIGHT;

            let x_min = x - h_radius;
            let x_max = x + h_radius;
            let y_min = y - v_radius;
            let y_max = y + v_radius;

            let obj_list: Vec<GDObject> = parsed
                .objects
                .iter()
                .filter(|o| {
                    let x = o[&2].parse::<f32>().unwrap();
                    let y = o[&3].parse::<f32>().unwrap();

                    x >= x_min && x < x_max && y >= y_min && y < y_max
                })
                .filter_map(|o| {
                    let o = replace_obj_id(o.clone());
                    if o.get(&129)
                        .unwrap_or(&"1.0".to_string())
                        .parse::<f32>()
                        .unwrap()
                        < 0.01
                        || o.get(&128)
                            .unwrap_or(&"1.0".to_string())
                            .parse::<f32>()
                            .unwrap()
                            < 0.01
                    {
                        None
                    } else if !all_avaliable_ids.contains(&o[&1].parse::<u16>().unwrap()) {
                        missing
                            .entry(set_i - 1)
                            .or_insert_with(HashSet::new)
                            .insert(o[&1].parse::<u16>().unwrap());
                        None
                    } else {
                        Some(to_gdobject(&o, x, y, parsed))
                    }
                })
                .collect();

            //println!("{} ({}): {} objs", digit_set, digit, obj_list.len());

            DigitObjects { objs: obj_list }
        })
    });

    // print missing
    for (set, missing) in missing {
        println!("{}:", get_creator_name(set));
        for id in missing {
            println!("missing {}", id);
        }
    }

    let bg = parse_gmd_file(include_str!(
        "../../../rust-shared/src/countdowndigits/bgnew.gmd"
    ));

    let bg_x = -7.0 * 30.0;
    let bg_y = -5.0 * 30.0;

    let days_marker = DigitObjects {
        objs: bg
            .objects
            .iter()
            .filter(|o| o.get(&57).map(String::as_ref) == Some("1"))
            .map(|o| to_gdobject(o, bg_x, bg_y, &bg))
            .collect(),
    };

    let hours_colon_deco = DigitObjects {
        objs: bg
            .clone()
            .objects
            .iter()
            .filter(|o| o.get(&57).map(String::as_ref) == Some("2"))
            .map(|o| to_gdobject(o, bg_x, bg_y, &bg))
            .collect(),
    };

    let minutes_colon_deco = DigitObjects {
        objs: bg
            .objects
            .iter()
            .filter(|o| o.get(&57).map(String::as_ref) == Some("3"))
            .map(|o| to_gdobject(o, bg_x, bg_y, &bg))
            .collect(),
    };

    let mut colons_missing_objects = HashSet::new();

    let hours_colon = [4, 6, 7, 8, 10, 11].map(|group| DigitObjects {
        objs: bg
            .objects
            .iter()
            .filter(|o| o.get(&57) == Some(&group.to_string()))
            .map(|o| to_gdobject(o, bg_x, bg_y, &bg))
            .collect(),
    });

    let minutes_colon = [12, 14, 15, 16, 18, 19].map(|group| DigitObjects {
        objs: bg
            .objects
            .iter()
            .filter(|o| o.get(&57) == Some(&group.to_string()))
            .map(|o| {
                if !all_avaliable_ids.contains(&o[&1].parse::<u16>().unwrap()) {
                    colons_missing_objects.insert(o[&1].parse::<u16>().unwrap());
                }
                to_gdobject(o, bg_x, bg_y, &bg)
            })
            .collect(),
    });

    for id in colons_missing_objects {
        println!("missing colon object {}", id);
    }

    let mut writer = Cursor::new(Vec::new());
    CountdownDigitSets(
        sets.map(|v| DigitSet(v)),
        days_marker,
        hours_colon_deco,
        minutes_colon_deco,
        hours_colon,
        minutes_colon,
    )
    .write(&mut writer)
    .unwrap();
    writer.into_inner()
}

fn replace_obj_id(mut o: HashMap<u16, String>) -> HashMap<u16, String> {
    let new_id = match o.get(&1).unwrap().as_ref() {
        "955" => "211",

        "150" => "3635",
        "133" => "3633",
        "460" => "3631",
        "494" => "3632",
        "50" => "3621",

        a => a,
    };
    o.insert(1, String::from(new_id));
    o
}

fn to_gdobject(
    o: &HashMap<u16, String>,
    x: f32,
    y: f32,
    parsed: &rust_shared::countdown::LevelParseResult,
) -> GDObject {
    let mut rotation = o
        .get(&6)
        .map(|v| v.parse::<f32>().unwrap())
        .unwrap_or(0.0)
        .to_radians();

    let h_flip = o.get(&4).map(|a| a.as_ref()) == Some("1");
    let v_flip = o.get(&5).map(|a| a.as_ref()) == Some("1");

    let id = o[&1].parse().unwrap();
    if id == 1704 || id == 1751 {
        if rotation > 360.0 - 70.0 {
            rotation = rotation - 360.0
        }
    }

    let x_scale = o
        .get(&128)
        .unwrap_or(&String::from("1.0"))
        .parse::<f32>()
        .unwrap();
    let mut x_warp_angle = -o
        .get(&132)
        .unwrap_or(&String::from("0.0"))
        .parse::<f32>()
        .unwrap()
        .to_radians()
        - rotation;
    let y_scale = o
        .get(&129)
        .unwrap_or(&String::from("1.0"))
        .parse::<f32>()
        .unwrap();
    let mut y_warp_angle = -o
        .get(&131)
        .unwrap_or(&String::from("0.0"))
        .parse::<f32>()
        .unwrap()
        .to_radians()
        + std::f32::consts::PI / 2.0
        - rotation;

    if h_flip {
        x_warp_angle += std::f32::consts::PI;
    }

    if v_flip {
        y_warp_angle += std::f32::consts::PI;
    }

    let [ix, iy, jx, jy] = get_transform(x_scale, x_warp_angle, y_scale, y_warp_angle);

    let default_color = GDColor {
        r: 255,
        g: 255,
        b: 255,
        opacity: 255,
        blending: false,
    };

    fn parse_hsv(hsv: &str) -> (f64, f64, f64, bool, bool) {
        let mut parts = hsv.split("a");

        let h = parts.next().unwrap().parse::<f64>().unwrap();
        let s = parts.next().unwrap().parse::<f64>().unwrap();
        let v = parts.next().unwrap().parse::<f64>().unwrap();

        let s_checked = parts.next().unwrap() == "1";
        let v_checked = parts.next().unwrap() == "1";

        (h, s, v, s_checked, v_checked)
    }

    let hsv1 =
        (o.get(&41).map(String::as_ref) == Some("1")).then(|| parse_hsv(o.get(&43).unwrap()));

    let hsv2 =
        (o.get(&42).map(String::as_ref) == Some("1")).then(|| parse_hsv(o.get(&44).unwrap()));

    GDObject {
        id,
        x: o[&2].parse::<f32>().unwrap() - x,
        y: o[&3].parse::<f32>().unwrap() - y,
        ix,
        iy,
        jx,
        jy,
        z_layer: o
            .get(&24)
            .map(|v| ZLayer::from_gd_num(v.parse::<i8>().unwrap()))
            .unwrap_or_else(|| OBJECT_DEFAULT_Z.get(&id).map(|v| v.0).unwrap_or(ZLayer::B1)),
        // z_layer: ZLayer::B3,
        z_order: o
            .get(&25)
            .map(|v| v.parse::<i32>().unwrap().min(128).max(-128) as i8)
            .unwrap_or_else(|| OBJECT_DEFAULT_Z.get(&id).map(|v| v.1).unwrap_or(0)),
        main_color: o
            .get(&21)
            .map(|c| {
                apply_hsv(
                    parsed
                        .colors
                        .get(&c.parse().unwrap())
                        .unwrap_or(&default_color)
                        .clone(),
                    hsv1,
                )
            })
            .unwrap_or(default_color),
        detail_color: o
            .get(&22)
            .map(|c| {
                apply_hsv(
                    parsed
                        .colors
                        .get(&c.parse().unwrap())
                        .unwrap_or(&default_color)
                        .clone(),
                    hsv2,
                )
            })
            .unwrap_or(default_color),
    }
}

fn apply_hsv(color: GDColor, hsv: Option<(f64, f64, f64, bool, bool)>) -> GDColor {
    if let Some((h, s, v, s_checked, v_checked)) = hsv {
        use color_space::{Hsv, Rgb};

        let rgb = Rgb::new(color.r as f64, color.g as f64, color.b as f64);
        let hsv = Hsv::from(rgb);
        let modified = Hsv::new(
            (hsv.h + h).rem_euclid(360.0),
            if s_checked {
                (hsv.s + s).min(1.0).max(0.0)
            } else {
                hsv.s * s
            },
            if v_checked {
                (hsv.v + v).min(1.0).max(0.0)
            } else {
                hsv.v * v
            },
        );
        let rgb_m = Rgb::from(modified);
        //println!("juh {:?}", rgb_m);
        // dbg!(rgb, rgb_m);
        GDColor::new(
            rgb_m.r as u8,
            rgb_m.g as u8,
            rgb_m.b as u8,
            color.opacity,
            color.blending,
        )
    } else {
        return color;
    }
}

pub fn get_transform(x_scale: f32, x_angle: f32, y_scale: f32, y_angle: f32) -> [f32; 4] {
    [
        x_scale * x_angle.cos(),
        x_scale * x_angle.sin(),
        y_scale * y_angle.cos(),
        y_scale * y_angle.sin(),
    ]
}
