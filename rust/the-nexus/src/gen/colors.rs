use itertools::Itertools;
use serde_json::json;

use crate::{
    gen::config::{available_hues, PICKER_COLUMNS, PICKER_ROWS},
    util::hsv_to_rgb,
};

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
                                        (1.0 - r as f32
                                            / (PICKER_ROWS as f32 - if c == 0 { 1.0 } else { 0.0 })).powf(1.5),
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
