use glam::vec4;

use crate::{
    log,
    object::{GDColor, GDObject},
    state::State,
};

use super::{billy::Billy, draw_obj_simple};

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

    draw_obj_simple(
        billy,
        &GDObject {
            id: 1597,
            x: 300.0,
            y: 300.0,
            ix: 1.0,
            iy: 0.0,
            jx: 0.0,
            jy: 1.0,
            ..Default::default()
        },
        false,
        vec4(0.0, 1.0, 0.0, 1.0),
    );
}
