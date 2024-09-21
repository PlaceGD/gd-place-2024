use core::fmt;
use std::{array, collections::HashMap, io::Cursor};

use binrw::BinWrite;
use itertools::Itertools;
use rust_shared::{
    countdown::{
        CountdownDigitSets, DigitObjects, DigitSet, DIGIT_HEIGHT, DIGIT_SETS, DIGIT_WIDTH,
    },
    gd::{
        layer::ZLayer,
        object::{GDColor, GDObject},
    },
};

use crate::objects::list::parse_gmd_file;

pub fn make_get_countdown_digits_fn() -> Vec<u8> {
    let mut parsed = parse_gmd_file(include_str!("../objects/countdowndigits.gmd"));

    parsed
        .objects
        .retain(|o| o.get(&20).map(|v| v.parse::<u16>().unwrap() >= 8) == Some(true));

    let x_offset = 8.0 * 30.0;
    let y_offset = 15.0 * 30.0;

    let h_radius = DIGIT_WIDTH / 2.0;
    let v_radius = DIGIT_HEIGHT / 2.0;

    let sets: [[DigitObjects; 10]; DIGIT_SETS] = array::from_fn(|digit_set| {
        array::from_fn(|digit| {
            let x = x_offset + digit as f32 * DIGIT_WIDTH;
            let y = y_offset + digit_set as f32 * DIGIT_HEIGHT;

            let x_min = x - h_radius;
            let x_max = x + h_radius;
            let y_min = y - v_radius;
            let y_max = y + v_radius;

            let objects = parsed
                .objects
                .iter()
                .filter(|o| {
                    let x = o[&2].parse::<f32>().unwrap();
                    let y = o[&3].parse::<f32>().unwrap();

                    x >= x_min && x < x_max && y >= y_min && y < y_max
                })
                .collect::<Vec<_>>();

            let obj_list: Vec<GDObject> = objects
                .iter()
                .map(|o| {
                    let rotation = o
                        .get(&6)
                        .map(|v| v.parse::<f32>().unwrap())
                        .unwrap_or(0.0)
                        .to_radians();

                    let h_flip = o.get(&4).map(|a| a.as_ref()) == Some("1");
                    let v_flip = o.get(&5).map(|a| a.as_ref()) == Some("1");

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

                    let [ix, iy, jx, jy] =
                        get_transform(x_scale, x_warp_angle, y_scale, y_warp_angle);

                    let default_color = GDColor {
                        r: 255,
                        g: 255,
                        b: 255,
                        opacity: 255,
                        blending: false,
                    };

                    GDObject {
                        id: o[&1].parse().unwrap(),
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
                            .map(|c| parsed.colors[&c.parse().unwrap()].clone())
                            .unwrap_or(default_color),
                        detail_color: o
                            .get(&22)
                            .map(|c| parsed.colors[&c.parse().unwrap()].clone())
                            .unwrap_or(default_color),
                    }
                })
                .collect();

            println!("{} ({}): {} objs", digit_set, digit, obj_list.len());

            // sorting happens at runtime because animations
            // obj_list.sort_by(|a, b| {
            //     // place blending objects at bottom of layer (dont care about detail color for this)
            //     let a_z = if a.main_color.blending {
            //         i8::MIN
            //     } else {
            //         a.z_order
            //     };
            //     let b_z = if b.main_color.blending {
            //         i8::MIN
            //     } else {
            //         b.z_order
            //     };
            //     (a.z_layer as u8)
            //         .cmp(&(b.z_layer as u8))
            //         .then(a_z.cmp(&b_z))
            // });

            DigitObjects { objs: obj_list }
        })
    });

    let mut writer = Cursor::new(Vec::new());
    CountdownDigitSets(sets.map(|v| DigitSet(v)))
        .write(&mut writer)
        .unwrap();
    writer.into_inner()
}

pub fn get_transform(x_scale: f32, x_angle: f32, y_scale: f32, y_angle: f32) -> [f32; 4] {
    [
        x_scale * x_angle.cos(),
        x_scale * x_angle.sin(),
        y_scale * y_angle.cos(),
        y_scale * y_angle.sin(),
    ]
}
