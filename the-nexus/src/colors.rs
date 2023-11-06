use itertools::Itertools;
use serde_json::json;

// 0-360
pub fn available_hues() -> impl IntoIterator<Item = u16> {
    [0, 30, 60, 80, 120, 160, 180, 210, 240, 270, 300, 330]
}

// brightness
pub const PICKER_ROWS: u32 = 4;

// saturation
pub const PICKER_COLUMNS: u32 = 4;

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h * 6.0;

    let c = v * s;
    let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());

    let (r, g, b) = if (0.0..1.0).contains(&h) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let m = v - c;
    let (r, g, b) = (r + m, g + m, b + m);

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

pub fn get_available_colors() -> serde_json::Value {
    json!({
        "rows": PICKER_ROWS,
        "columns": PICKER_COLUMNS,
        "hues": available_hues().into_iter().count(),
        "list": available_hues().into_iter()
            .map(|hue| {
                json!({
                    "hue": hue as f32,
                    "palette": (0..PICKER_ROWS)
                    .map(|r| {
                        (0..PICKER_COLUMNS)
                            .map(|c| {
                                hsv_to_rgb(
                                    hue as f32 / 360.0,
                                    c as f32 / (PICKER_COLUMNS as f32 - 1.0),
                                    1.0 - r as f32
                                        / (PICKER_ROWS as f32 - if c == 0 { 1.0 } else { 0.0 }),
                                )
                            })
                            .collect_vec()
                    })
                    .collect_vec()
                })
            })
            .collect_vec()
    })
}
