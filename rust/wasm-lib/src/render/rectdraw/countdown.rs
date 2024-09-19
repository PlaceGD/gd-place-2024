use std::{io::Cursor, sync::LazyLock};

use binrw::BinRead;
use glam::vec4;
use rust_shared::{console_log, countdown::CountdownDigitSets, gd::object::GDObject};

use crate::state::State;

use super::{billy::Billy, draw_obj_simple};

pub static COUNTDOWN_DIGITS: LazyLock<CountdownDigitSets> = LazyLock::new(|| {
    console_log!("JONK");
    let bytes = include_bytes!("../../countdown_digits");

    CountdownDigitSets::read(&mut Cursor::new(bytes)).unwrap()
});

pub fn draw(state: &mut State, billy: &mut Billy) {
    if state.event_elapsed >= 0.0 {
        return;
    }
    billy.apply_transform(state.view_transform());

    let time_until = -state.event_elapsed / 1000.0;

    let days = (time_until / 86400.0).floor() as i32;
    let hours = ((time_until - (days as f64 * 86400.0)) / 3600.0).floor() as i32;
    let minutes =
        ((time_until - (days as f64 * 86400.0) - (hours as f64 * 3600.0)) / 60.0).floor() as i32;
    let seconds =
        (time_until - (days as f64 * 86400.0) - (hours as f64 * 3600.0) - (minutes as f64 * 60.0))
            .floor() as i32;
    let line1 = format!("{:02}", days);
    let line2 = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    let mut offset = glam::vec2(450.0 + 30.0 * 7.0 * 2.0 + 120.0, 450.0 + 14.0 * 30.0);
    let mut index = 0;
    for c in line1.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        draw_digit(1, digit, state, billy, offset);
        offset.x += 30.0 * 7.0;
        index += 1;
    }
    let mut offset = glam::vec2(450.0, 450.0);
    let mut set = 0;
    for c in line2.chars() {
        if c == ':' {
            offset.x += 120.0;
            set += 1;
            continue;
        }
        let digit = c.to_digit(10).unwrap() as usize;
        draw_digit(set % 3, digit, state, billy, offset);
        offset.x += 30.0 * 7.0;
        index += 1;
    }
}

pub fn draw_digit(set: usize, digit: usize, state: &State, billy: &mut Billy, offset: glam::Vec2) {
    let digit = &COUNTDOWN_DIGITS.0[set].0[digit];

    for layer in &digit.layers {
        //log!("layer: {:?}", objects.len());

        for obj in &layer.objs {
            draw_obj_simple(
                billy,
                &obj.offset(offset),
                true,
                vec4(
                    obj.detail_color.r as f32 / 255.0,
                    obj.detail_color.g as f32 / 255.0,
                    obj.detail_color.b as f32 / 255.0,
                    obj.detail_color.opacity as f32 / 255.0,
                ),
                obj.detail_color.blending,
            );
            draw_obj_simple(
                billy,
                &obj.offset(offset),
                false,
                vec4(
                    obj.main_color.r as f32 / 255.0,
                    obj.main_color.g as f32 / 255.0,
                    obj.main_color.b as f32 / 255.0,
                    obj.main_color.opacity as f32 / 255.0,
                ),
                obj.main_color.blending,
            );
        }
    }
}

pub struct CountdownDigit {
    pub objects: Vec<Vec<GDObject>>,
}

impl Default for CountdownDigit {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}
