use glam::{vec2, vec4, Vec4};
use rust_shared::{console_log, gd::layer::Z_LAYERS, map};

use crate::{
    object::GDObjectExt,
    state::State,
    utilgen::{DETAIL_SPRITES, MAIN_SPRITES, OBJECT_INFO},
};

use super::{
    billy::{Billy, BlendMode},
    draw_level, draw_level_obj_sprite,
};

// draw everything after the background and grid here (those rdone separately for REASONS .)
pub fn draw(state: &mut State, billy: &mut Billy) {
    billy.apply_transform(state.view_transform());

    if state.show_preview {
        state
            .level
            .add_object(state.preview_object.into_obj(), [255; 20]);
    }

    let selected_color = |lighter| {
        let c = map!(
            (state.time * 10.0).sin(),
            -1.0,
            1.0,
            if lighter {
                150.0 / 255.0
            } else {
                100.0 / 255.0
            },
            if lighter { 1.0 } else { 200.0 / 255.0 }
        );
        vec4(1.0, c, c, 1.0)
    };

    draw_level(state, billy, &state.level, |k, _, detail| {
        (state.selected_object == Some(k)).then(|| selected_color(detail))
    });

    if state.show_preview {
        state.level.remove_object([255; 20]);
    }

    // for &layer in Z_LAYERS {
    //     billy.set_blend_mode(if state.show_collidable {
    //         BlendMode::Normal
    //     } else {
    //         BlendMode::Additive
    //     });
    //     for z_order in -50..=50 {
    //         state.level.foreach_obj_in_z(
    //             layer,
    //             z_order,
    //             |key, obj| {
    //                 if let Some(sprite) = MAIN_SPRITES[obj.id as usize] {
    //                     if obj.main_color.blending {
    //                         let color = if state.selected_object == Some(key) {
    //                             selected_color(false)
    //                         } else {
    //                             Vec4::from_array(
    //                                 [
    //                                     obj.main_color.r,
    //                                     obj.main_color.g,
    //                                     obj.main_color.b,
    //                                     obj.main_color.opacity,
    //                                 ]
    //                                 .map(|v| v as f32 / 255.0),
    //                             )
    //                         };
    //                         draw_level_obj_sprite(state, billy, sprite, obj, color);
    //                     }
    //                 }
    //             },
    //             state.show_preview.then(|| state.preview_object.into_obj()),
    //         )
    //     }

    //     billy.set_blend_mode(BlendMode::Normal);
    //     for z_order in -50..=50 {
    //         state.level.foreach_obj_in_z(
    //             layer,
    //             z_order,
    //             |key, obj| {
    //                 if let Some(sprite) = MAIN_SPRITES[obj.id as usize] {
    //                     if !obj.main_color.blending {
    //                         let color = if state.selected_object == Some(key) {
    //                             selected_color(false)
    //                         } else {
    //                             Vec4::from_array(
    //                                 [
    //                                     obj.main_color.r,
    //                                     obj.main_color.g,
    //                                     obj.main_color.b,
    //                                     obj.main_color.opacity,
    //                                 ]
    //                                 .map(|v| v as f32 / 255.0),
    //                             )
    //                         };
    //                         draw_level_obj_sprite(state, billy, sprite, obj, color);
    //                     }
    //                 }
    //                 if let Some(sprite) = DETAIL_SPRITES[obj.id as usize] {
    //                     if !obj.detail_color.blending {
    //                         let color = if state.selected_object == Some(key) {
    //                             selected_color(true)
    //                         } else {
    //                             Vec4::from_array(
    //                                 [
    //                                     obj.detail_color.r,
    //                                     obj.detail_color.g,
    //                                     obj.detail_color.b,
    //                                     obj.detail_color.opacity,
    //                                 ]
    //                                 .map(|v| v as f32 / 255.0),
    //                             )
    //                         };
    //                         draw_level_obj_sprite(state, billy, sprite, obj, color);
    //                     }
    //                 }
    //             },
    //             state.show_preview.then(|| state.preview_object.into_obj()),
    //         )
    //     }

    //     billy.set_blend_mode(if state.show_collidable {
    //         BlendMode::Normal
    //     } else {
    //         BlendMode::Additive
    //     });
    //     for z_order in -50..=50 {
    //         state.level.foreach_obj_in_z(
    //             layer,
    //             z_order,
    //             |key, obj| {
    //                 if let Some(sprite) = DETAIL_SPRITES[obj.id as usize] {
    //                     if obj.detail_color.blending {
    //                         let color = if state.selected_object == Some(key) {
    //                             selected_color(false)
    //                         } else {
    //                             Vec4::from_array(
    //                                 [
    //                                     obj.detail_color.r,
    //                                     obj.detail_color.g,
    //                                     obj.detail_color.b,
    //                                     obj.detail_color.opacity,
    //                                 ]
    //                                 .map(|v| v as f32 / 255.0),
    //                             )
    //                         };
    //                         draw_level_obj_sprite(state, billy, sprite, obj, color);
    //                     }
    //                 }
    //             },
    //             state.show_preview.then(|| state.preview_object.into_obj()),
    //         )
    //     }
    // }

    // selection box
    if !state.hide_outline {
        billy.set_blend_mode(BlendMode::Normal);
        let highlight_obj = if state.show_preview {
            Some((state.preview_object.into_obj(), (100, 255, 100), None))
        } else if let Some(d) = state.selected_object {
            state.level.get_obj_by_key(d).map(|v| {
                (
                    *v,
                    (255, 100, 100),
                    Some(String::from_utf8(d.into()).unwrap()),
                )
            })
        } else {
            None
        };

        if let Some((obj, (r, g, b), _)) = highlight_obj {
            // let transform = state.view_transform() * obj_transform(&obj);

            let scale_vec = vec2(
                OBJECT_INFO[obj.id as usize].builtin_scale_x,
                OBJECT_INFO[obj.id as usize].builtin_scale_y,
            );

            let rect = obj.padded_rect(0.0);
            let rect_size_vec = vec2(rect.w, rect.h);

            let old_t = billy.get_transform();
            billy.apply_transform(obj.transform());

            billy.centered_dashed_rect(
                vec2(0.0, 0.0),
                rect_size_vec + vec2(30.0, 30.0) / scale_vec,
                vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 0.75),
                rect_size_vec * scale_vec + 30.0,
            );

            billy.set_transform(old_t);

            // rects.push(pipeline_rect::instance::Instance::new(
            //     transform
            //         * Affine2::from_translation(
            //             -(rect_size_vec + vec2(30.0, 30.0) / scale_vec) / 2.0,
            //         )
            //         * Affine2::from_scale(rect_size_vec + vec2(30.0, 30.0) / scale_vec),
            //     vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 0.75),
            //     1000,
            //     vec2(0.0, 0.0),
            //     rect_size_vec * scale_vec + 30.0,
            // ));
        }
    }

    // ground
    if !state.hide_ground {
        billy.set_blend_mode(BlendMode::Normal);
        const GROUND_SIZE_BLOCKS: f32 = 4.25;
        const GROUND_SIZE_UNITS: f32 = GROUND_SIZE_BLOCKS * 30.0;

        let view_rect = state.get_camera_world_rect();
        let min_x = (view_rect.x / GROUND_SIZE_UNITS).floor() as i32 - 1;
        let max_x = ((view_rect.x + view_rect.w) / GROUND_SIZE_UNITS).floor() as i32 + 1;

        for i in min_x..=max_x {
            let x = i as f32 * GROUND_SIZE_BLOCKS * 30.0;
            // let t = transform
            //     * Affine2::from_scale_angle_translation(
            //         vec2(GROUND_SIZE_UNITS, GROUND_SIZE_UNITS) + 0.2,
            //         0.0,
            //         vec2(x, -GROUND_SIZE_UNITS) - 0.1,
            //     );

            billy.textured_rect(
                vec2(x, -GROUND_SIZE_UNITS),
                vec2(GROUND_SIZE_UNITS, GROUND_SIZE_UNITS),
                vec4(
                    state.ground1_color.0 as f32 / 255.0,
                    state.ground1_color.1 as f32 / 255.0,
                    state.ground1_color.2 as f32 / 255.0,
                    1.0,
                ),
                1,
                vec2(0.0, 0.0),
                vec2(256.0, 256.0),
            );
            billy.textured_rect(
                vec2(x, -GROUND_SIZE_UNITS),
                vec2(GROUND_SIZE_UNITS, GROUND_SIZE_UNITS),
                vec4(
                    state.ground2_color.0 as f32 / 255.0,
                    state.ground2_color.1 as f32 / 255.0,
                    state.ground2_color.2 as f32 / 255.0,
                    1.0,
                ),
                1,
                vec2(0.0, 256.0),
                vec2(256.0, 256.0),
            );
        }
        billy.set_blend_mode(BlendMode::Additive);
        billy.centered_textured_rect(
            vec2(state.camera_pos.x, -1.0),
            vec2(GROUND_SIZE_UNITS * 10.0, 2.0),
            vec4(1.0, 1.0, 1.0, 1.0),
            1,
            vec2(0.0, 513.0),
            vec2(256.0, 1.0),
        );
    };
}
