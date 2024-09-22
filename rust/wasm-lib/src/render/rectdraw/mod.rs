use billy::Billy;
use glam::{uvec2, vec2, vec4, Vec4};
use rust_shared::{
    gd::{object::GDObject, special_ids, HitboxType, ObjectCategory},
    sprite::SpriteInfo,
};

use crate::{
    object::GDObjectExt,
    state::State,
    utilgen::{DETAIL_SPRITES, MAIN_SPRITES, OBJECT_INFO, SFX_ICON_SPRITES},
};

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

pub fn draw_level_obj_sprite(
    state: &State,
    billy: &mut Billy,
    sprite: SpriteInfo,
    obj: &GDObject,
    color: Vec4,
) {
    if state.hide_triggers && special_ids::TRIGGERS.contains(&obj.id) {
        return;
    }

    let info = OBJECT_INFO[obj.id as usize];

    let old_t = billy.get_transform();
    billy.apply_transform(obj.transform());

    let tex_idx = if info.builtin_scale_x == 1.0 && info.builtin_scale_y == 1.0 {
        2
    } else {
        3
    };

    let uv_pos = uvec2(sprite.pos.0, sprite.pos.1).as_vec2();
    let uv_size = uvec2(sprite.size.0, sprite.size.1).as_vec2();

    let tint_color = if !state.show_collidable {
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
    }

    billy.set_transform(old_t);
}
