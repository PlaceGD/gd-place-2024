use billy::{Billy, BlendMode};
use glam::{uvec2, vec2, vec3, vec4, Vec4};
use rust_shared::{
    console_log,
    gd::{layer::Z_LAYERS, object::GDObject, special_ids, HitboxType, ObjectCategory},
    map,
    sprite::SpriteInfo,
    util::now,
};
use std::{
    f32::consts::PI,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
    level::{DbKey, Level},
    object::GDObjectExt,
    state::State,
    utilgen::{
        DETAIL_SPRITES, MAIN_SPRITES, OBJECT_INFO, OBJECT_MAIN_OVER_DETAIL_IDS, SFX_ICON_SPRITES,
        SONG_ICON_SPRITES,
    },
};

const fn object_main_over_detail() -> [bool; 4600] {
    let mut arr = [false; 4600];
    let ids = OBJECT_MAIN_OVER_DETAIL_IDS;
    let mut i = 0;
    while i < ids.len() {
        arr[ids[i] as usize] = true;
        i += 1;
    }
    arr
}

pub const OBJECT_MAIN_OVER_DETAIL: [bool; 4600] = object_main_over_detail();

pub mod billy;
pub mod countdown;
pub mod level;

pub fn draw_obj_simple(
    billy: &mut Billy,
    obj: &GDObject,
    detail: bool,
    color: Vec4,
    blending: bool,
) {
    if let Some(sprite) = if detail {
        &DETAIL_SPRITES
    } else {
        &MAIN_SPRITES
    }[obj.id as usize]
    {
        let info = OBJECT_INFO[obj.id as usize];

        let old_t = billy.get_transform();
        billy.apply_transform({
            // let scale_x = OBJECT_INFO[this.id as usize].builtin_scale_x / 4.0;
            // let scale_y = OBJECT_INFO[this.id as usize].builtin_scale_y / 4.0;

            glam::Affine2::from_mat2_translation(
                glam::mat2(
                    vec2(obj.ix / 4.0, obj.iy / 4.0),
                    vec2(obj.jx / 4.0, obj.jy / 4.0),
                ),
                vec2(obj.x, obj.y),
            )
        });

        let tex_idx = if info.builtin_scale_x == 1.0 && info.builtin_scale_y == 1.0 {
            2
        } else {
            3
        };

        if blending {
            billy.set_blend_mode(billy::BlendMode::Additive);
        } else {
            billy.set_blend_mode(billy::BlendMode::Normal);
        }

        let uv_pos = uvec2(sprite.pos.0, sprite.pos.1).as_vec2();
        let uv_size = uvec2(sprite.size.0, sprite.size.1).as_vec2();

        billy.centered_textured_rect(
            vec2(sprite.offset.0, -sprite.offset.1),
            uv_size,
            color,
            tex_idx,
            uv_pos,
            uv_size,
        );

        billy.set_transform(old_t);
    }
}

fn ease_out_expo(x: f32) -> f32 {
    if x == 1.0 {
        1.0
    } else {
        1.0 - 2.0f32.powf(-10.0 * x)
    }
}

use glam::Vec3Swizzles;

pub fn draw_level_obj_sprite<K: Default + Hash + Eq + Copy>(
    state: &State,
    billy: &mut Billy,
    sprite: SpriteInfo,
    obj: &GDObject,
    color: Vec4,
    key: K,
) {
    if state.hide_triggers && special_ids::TRIGGERS.contains(&obj.id) {
        return;
    }

    let end_anim_time = ((state.now - state.event_end) / 1000.0) as f32;

    let info = OBJECT_INFO[obj.id as usize];

    let old_t = billy.get_transform();
    billy.apply_transform(obj.transform());

    let mut tint_color = vec4(1.0, 1.0, 1.0, 1.0);

    if end_anim_time > 0.0 {
        let explosion_time = 10.0;
        let end_anim_time = end_anim_time;
        let dist_from_center = (vec2(obj.x, obj.y) - state.camera_pos).length();
        let delay = dist_from_center * 0.0001;
        let explosion_d =
            ease_out_expo(((end_anim_time - explosion_time) / 3.0 - delay).clamp(0.0, 1.0));
        let obj_z = ((obj.z_order as f32 / 256.0 + obj.z_layer as usize as f32) / 9.0) * 2.0 - 1.0;

        let explode_vec =
            vec3(obj.x, obj.y, obj_z * 100.0) - vec3(state.camera_pos.x, state.camera_pos.y, 0.0);

        let explode_dir = explode_vec.normalize();
        let explode_strength = 1.0 / explode_vec.length().max(30.0).min(900.0) + 0.5;

        let random_x = random_num(key, 0) * 2.0 - 1.0;
        let random_y = random_num(key, 1) * 2.0 - 1.0;
        let random_z = random_num(key, 2) * 2.0 - 1.0;

        let angular_velocity = random_num(key, 1) * 2.0 - 1.0;

        // billy
        //     .translate((vec2(obj.x, obj.y) - state.camera_pos) * explosion_d * random_offset * 3.0);
        let pos = vec2(obj.x, obj.y)
            + (vec2(random_x, random_y) * 90.0 + explode_dir.xy() * explode_strength * 900.0)
                * explosion_d;
        let new_z = random_z * explosion_d + 1.2;
        let z_scaleup = 4.0 - 3.0 / (1.0 + (random_z + 1.0) * explosion_d);
        let new_pos = state.camera_pos + (pos - state.camera_pos) * z_scaleup;

        billy.translate(new_pos - vec2(obj.x, obj.y));
        billy.scale(vec2(z_scaleup, z_scaleup));
        billy.rotate(
            angular_velocity * PI * 0.1 * (end_anim_time - explosion_time - delay).max(0.0)
                + angular_velocity * explosion_d,
        );

        let fadeout_d =
            1.0 - ((end_anim_time - 3.0 - dist_from_center * 0.001) / 10.0).clamp(0.0, 1.0);
        tint_color.w = tint_color.w * 0.2 + fadeout_d * 0.8;
    }

    let tex_idx = if info.builtin_scale_x == 1.0 && info.builtin_scale_y == 1.0 {
        2
    } else {
        3
    };

    let uv_pos = uvec2(sprite.pos.0, sprite.pos.1).as_vec2();
    let uv_size = uvec2(sprite.size.0, sprite.size.1).as_vec2();

    tint_color *= if !state.show_collidable {
        if info.category == ObjectCategory::Triggers {
            vec4(1.0, 1.0, 1.0, 1.0)
        } else {
            color
        }
    } else {
        match info.hitbox_type {
            HitboxType::NoHitbox => vec4(0.0, 0.0, 0.0, 0.0),
            HitboxType::Solid => vec4(0.0, 0.0, 100.0, 0.5),
            HitboxType::Hazard => vec4(100.0, 0.0, 0.0, 0.5),
            HitboxType::Special => vec4(0.0, 100.0, 0.0, 0.5),
        }
    };

    billy.centered_textured_rect(
        vec2(sprite.offset.0, -sprite.offset.1),
        uv_size,
        tint_color,
        tex_idx,
        uv_pos,
        uv_size,
    );

    if special_ids::COLOR_TRIGGERS.contains(&obj.id) {
        billy.centered_solid_rect(
            -vec2(0.0, 42.0),
            vec2(128.0, 128.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );
        if !state.show_collidable {
            billy.centered_solid_rect(-vec2(0.0, 42.0), vec2(112.0, 112.0), color);
        }
    }
    if special_ids::SFX_TRIGGER == obj.id {
        let sfx_id = obj.main_color.r;

        if let Some(sprite) = SFX_ICON_SPRITES.get(sfx_id as usize) {
            let uv_pos = uvec2(sprite.pos.0, sprite.pos.1).as_vec2();
            let uv_size = uvec2(sprite.size.0, sprite.size.1).as_vec2();
            billy.centered_textured_rect(
                -vec2(0.0, 42.0),
                uv_size * 128.0 / uv_size.max_element(),
                Vec4::ONE,
                2,
                uv_pos,
                uv_size,
            )
        }
    } else if special_ids::SONG_TRIGGER == obj.id {
        let song_id = obj.main_color.r;

        if let Some(sprite) = SONG_ICON_SPRITES.get(song_id as usize) {
            let uv_pos = uvec2(sprite.pos.0, sprite.pos.1).as_vec2();
            let uv_size = uvec2(sprite.size.0, sprite.size.1).as_vec2();
            billy.centered_textured_rect(
                -vec2(0.0, 42.0),
                uv_size * 128.0 / uv_size.max_element(),
                Vec4::ONE,
                2,
                uv_pos,
                uv_size,
            )
        }
    }

    billy.set_transform(old_t);
}

fn random_num<K: Default + Hash + Eq + Copy>(key: K, i: usize) -> f32 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    i.hash(&mut hasher);
    hasher.finish() as f32 / u64::MAX as f32
}

pub fn draw_level<K: Default + Hash + Eq + Copy>(
    state: &State,
    billy: &mut Billy,
    level: &Level<K>,
    mut color_override: impl FnMut(K, &GDObject, bool) -> Option<Vec4>,
) {
    for layer in 0..(Z_LAYERS.len() + 1) {
        for sheet_batch_idx in 0..5 {
            for batch_idx in 0..2 {
                billy.set_blend_mode(if state.show_collidable {
                    BlendMode::Normal
                } else {
                    [BlendMode::Additive, BlendMode::Normal][batch_idx]
                });
                for (_, chunk) in &level.chunks {
                    let sheet_batch = &chunk.layers[layer].sheet_batches[sheet_batch_idx];
                    let batch = &sheet_batch[batch_idx];

                    // console_log!("bend {}", i);
                    for (_, m) in batch {
                        for (key, (obj, draw)) in m {
                            for &bottom_texture in match draw {
                                crate::level::ObjectDraw::Both => &[false, true] as &[bool],
                                crate::level::ObjectDraw::TopTexture => &[false],
                                crate::level::ObjectDraw::BottomTexture => &[true],
                            } {
                                let main_over_detail = OBJECT_MAIN_OVER_DETAIL[obj.id as usize];
                                let bottom_texture = if main_over_detail {
                                    !bottom_texture
                                } else {
                                    bottom_texture
                                };
                                let (sprites, color) = if bottom_texture {
                                    (&DETAIL_SPRITES, obj.detail_color)
                                } else {
                                    (&MAIN_SPRITES, obj.main_color)
                                };
                                if let Some(sprite) = sprites[obj.id as usize] {
                                    // console_log!("-> {:?} {}", draw, batch_idx);
                                    if color.blending == (batch_idx == 0) {
                                        let color = color_override(*key, obj, bottom_texture)
                                            .unwrap_or(Vec4::from_array(
                                                [color.r, color.g, color.b, color.opacity]
                                                    .map(|v| v as f32 / 255.0),
                                            ));
                                        // let color = if state.selected_object == Some(*key) {
                                        //     selected_color(detail)
                                        // } else {
                                        //     Vec4::from_array(
                                        //         [color.r, color.g, color.b, color.opacity]
                                        //             .map(|v| v as f32 / 255.0),
                                        //     )
                                        // };
                                        draw_level_obj_sprite(
                                            state, billy, sprite, obj, color, *key,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
