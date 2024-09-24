use core::fmt;
use std::{array, collections::HashMap, io::Cursor};

use binrw::BinWrite;
use itertools::Itertools;
use rust_shared::{
    countdown::{
        get_countdown_sets, CountdownDigitSets, DigitObjects, DigitSet, DIGIT_HEIGHT, DIGIT_SETS,
        DIGIT_WIDTH,
    },
    gd::{
        layer::ZLayer,
        object::{GDColor, GDObject},
    },
};

use crate::objects::list::parse_gmd_file;

pub fn make_get_countdown_digits_fn() -> Vec<u8> {
    let (files, sets) = get_countdown_sets(parse_gmd_file);

    let x_offset = 8.0 * 30.0;
    let y_offset = 15.0 * 30.0;

    let h_radius = DIGIT_WIDTH / 2.0;
    let v_radius = DIGIT_HEIGHT / 2.0;

    let sets: [[DigitObjects; 10]; DIGIT_SETS] = sets.map(|set_ptr| {
        let parsed = &files[set_ptr.file];
        let digit_set = set_ptr.set;
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
                .map(|o| to_gdobject(o, x, y, parsed))
                .collect();

            println!("{} ({}): {} objs", digit_set, digit, obj_list.len());

            DigitObjects { objs: obj_list }
        })
    });

    let bg = parse_gmd_file(include_str!(
        "../../../rust-shared/src/countdowndigits/bg.gmd"
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

    let hours_colon = DigitObjects {
        objs: bg
            .objects
            .iter()
            .filter(|o| o.get(&57).map(String::as_ref) == Some("2"))
            .map(|o| to_gdobject(o, bg_x, bg_y, &bg))
            .collect(),
    };

    let minutes_colon = DigitObjects {
        objs: bg
            .objects
            .iter()
            .filter(|o| o.get(&57).map(String::as_ref) == Some("3"))
            .map(|o| to_gdobject(o, bg_x, bg_y, &bg))
            .collect(),
    };

    let mut writer = Cursor::new(Vec::new());
    CountdownDigitSets(
        sets.map(|v| DigitSet(v)),
        days_marker,
        hours_colon,
        minutes_colon,
    )
    .write(&mut writer)
    .unwrap();
    writer.into_inner()
}

fn to_gdobject(
    o: &HashMap<u16, String>,
    x: f32,
    y: f32,
    parsed: &rust_shared::countdown::LevelParseResult,
) -> GDObject {
    let rotation = o
        .get(&6)
        .map(|v| v.parse::<f32>().unwrap())
        .unwrap_or(0.0)
        .to_radians();

    let h_flip = o.get(&4).map(|a| a.as_ref()) == Some("1");
    let v_flip = o.get(&5).map(|a| a.as_ref()) == Some("1");

    let id = o[&1].parse().unwrap();

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

    fn parse_hsv(hsv: &str) -> (f64, f64, f64) {
        let mut parts = hsv.split("a");

        let h = parts.next().unwrap().parse::<f64>().unwrap();
        let s = parts.next().unwrap().parse::<f64>().unwrap();
        let v = parts.next().unwrap().parse::<f64>().unwrap();

        let s_checked = parts.next().unwrap() == "1";
        let v_checked = parts.next().unwrap() == "1";
        if s_checked || v_checked {
            panic!("hsv feature not supported");
        }

        (h, s, v)
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
        z_layer: ZLayer::from_gd_num(
            o.get(&24)
                .unwrap_or(&String::from("0"))
                .parse::<i8>()
                .unwrap(),
        ),
        // z_layer: ZLayer::B3,
        z_order: o
            .get(&25)
            .unwrap_or(&String::from("0"))
            .parse::<i8>()
            .unwrap(),
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

fn apply_hsv(color: GDColor, hsv: Option<(f64, f64, f64)>) -> GDColor {
    if let Some((h, s, v)) = hsv {
        use color_space::{Hsv, Rgb};

        let rgb = Rgb::new(color.r as f64, color.g as f64, color.b as f64);
        let hsv = Hsv::from(rgb);
        let modified = Hsv::new((hsv.h + h).rem_euclid(360.0), hsv.s * s, hsv.v * v);
        let rgb_m = Rgb::from(modified);
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
