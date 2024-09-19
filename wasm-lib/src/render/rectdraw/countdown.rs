use glam::vec4;

use crate::{
    log,
    object::{GDColor, GDObject},
    state::State,
};

use super::{billy::Billy, draw_obj_simple};

use crate::utilgen::COUNTDOWN_DIGITS;

pub fn draw(state: &State, billy: &mut Billy) {
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

    let text = format!("{:02}:{:02}:{:02}:{:02}", days, hours, minutes, seconds);
    let mut offset = glam::vec2(450.0, 450.0);
    for c in text.chars() {
        if c == ':' {
            offset.x += 120.0;
            continue;
        }
        let digit = c.to_digit(10).unwrap() as usize;
        draw_digit(1, digit, state, billy, offset);
        offset.x += 30.0 * 7.0;
    }
}

pub fn draw_digit(set: usize, digit: usize, state: &State, billy: &mut Billy, offset: glam::Vec2) {
    let layers = COUNTDOWN_DIGITS[set][digit];

    for layer in layers {
        let objects = layer
            .split(";")
            .filter(|a| !a.is_empty())
            .map(|t| GDObject::from_str(t))
            .collect::<Vec<_>>();

        //log!("layer: {:?}", objects.len());

        for obj in objects {
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
