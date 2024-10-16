use std::{mem, ptr};
use wasm_bindgen::prelude::*;

use glam::{mat2, vec2, Affine2, Vec2};
// use bytemuck::{bytes_of, Pod, Zeroable};
use rust_shared::{
    gd::{
        layer::ZLayer,
        level::{END_RADIUS, LEVEL_HEIGHT_UNITS, LEVEL_WIDTH_UNITS},
        object::{GDColor, GDObject},
        HitboxType,
    },
    util::Rect,
};

use crate::{
    level::ChunkCoord, util::get_max_bounding_box, utilgen::OBJECT_INFO, ErrorType, RustError,
};

pub type ObjectTupleForm = (
    u16,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    i8,
    i8,
    (u8, u8, u8, u8, bool),
    (u8, u8, u8, u8, bool),
);

pub trait GDObjectExt {
    fn transform(&self) -> Affine2;
    fn padded_rect(&self, pad: f32) -> Rect<f32>;
}

impl GDObjectExt for GDObject {
    fn transform(&self) -> Affine2 {
        let scale_x = OBJECT_INFO[self.id as usize].builtin_scale_x / 4.0;
        let scale_y = OBJECT_INFO[self.id as usize].builtin_scale_y / 4.0;

        Affine2::from_mat2_translation(
            mat2(
                vec2(self.ix * scale_x, self.iy * scale_x),
                vec2(self.jx * scale_y, self.jy * scale_y),
            ),
            vec2(self.x, self.y),
        )
    }
    fn padded_rect(&self, pad: f32) -> Rect<f32> {
        let mut rect_size = get_max_bounding_box(self.id as u32).unwrap_or((10.0, 10.0));
        // rect_size.0 *= OBJECT_INFO[obj.id as usize].builtin_scale_x;
        // rect_size.1 *= OBJECT_INFO[obj.id as usize].builtin_scale_y;

        rect_size.0 += pad;
        rect_size.1 += pad;

        Rect::new(
            -rect_size.0 / 2.0,
            -rect_size.1 / 2.0,
            rect_size.0,
            rect_size.1,
        )
    }
}

// IF THIS IS EVER CHANGED MAKE SURE TO CHANGE THE TYPESCRIPT TYPE IN SHAREDLIB
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
#[wasm_bindgen]
pub struct GDObjectOpt {
    pub id: u16,
    pub x: f32,
    pub y: f32,

    pub x_scale_exp: i8,
    pub x_angle: i8,

    pub y_scale_exp: i8,
    pub y_angle: i8,

    pub z_layer: ZLayer,
    pub z_order: i8,
    pub main_color: GDColor,
    pub detail_color: GDColor,
}

impl GDObjectOpt {
    pub fn into_obj(self) -> GDObject {
        let [ix, iy, jx, jy] = convert_opt_transform(
            self.x_scale_exp,
            self.x_angle,
            self.y_scale_exp,
            self.y_angle,
        );

        GDObject {
            id: self.id,
            x: self.x,
            y: self.y,
            ix,
            iy,
            jx,
            jy,
            z_layer: self.z_layer,
            z_order: self.z_order,
            main_color: self.main_color,
            detail_color: self.detail_color,
        }
    }
    pub fn fix(&mut self) {
        self.x_angle = self.x_angle.rem_euclid(72);
        self.y_angle = self.y_angle.rem_euclid(72);
        self.x_scale_exp = self.x_scale_exp.clamp(-12, 12);
        self.y_scale_exp = self.y_scale_exp.clamp(-12, 12);

        self.x = self.x.clamp(0.0, LEVEL_WIDTH_UNITS as f32 - 0.001);
        self.y = self.y.clamp(0.0, LEVEL_HEIGHT_UNITS as f32 - 0.001);

        let top_right = vec2(LEVEL_WIDTH_UNITS as f32, LEVEL_HEIGHT_UNITS as f32);
        let pos = vec2(self.x, self.y);

        let funny_len = |v: Vec2| (v.x.powf(4.0) + v.y.powf(4.0)).powf(1.0 / 4.0);
        let funny_norm = |v: Vec2| v / funny_len(v);

        if OBJECT_INFO[self.id as usize].hitbox_type != HitboxType::NoHitbox
            && funny_len(top_right - pos) < END_RADIUS as f32
        {
            let new_pos = top_right + funny_norm(pos - top_right) * END_RADIUS as f32 * 1.001;
            self.x = new_pos.x;
            self.y = new_pos.y;
        }
    }
    // pub fn debug_str(&self) -> String {
    //     format!("{:?}", self)
    // }
}

#[wasm_bindgen]
impl GDObjectOpt {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: u16,
        x: f32,
        y: f32,
        x_scale_exp: i8,
        x_angle: i8,
        y_scale_exp: i8,
        y_angle: i8,
        z_layer: ZLayer,
        z_order: i8,
        main_color: GDColor,
        detail_color: GDColor,
    ) -> Self {
        Self {
            id,
            x,
            y,
            z_layer,
            z_order,
            main_color,
            detail_color,
            x_scale_exp,
            x_angle,
            y_scale_exp,
            y_angle,
        }
    }
    pub fn bytes(&self) -> js_sys::Uint8Array {
        // SAFETY:
        // this is just getting the raw bytes of the struct. it will always be the correct size
        // and endianess
        let bytes: [u8; mem::size_of::<GDObjectOpt>()] =
            unsafe { mem::transmute(ptr::read(self as *const _)) };

        let array = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
        array.copy_from(&bytes);

        array
    }

    pub fn from_bytes(bytes: js_sys::Uint8Array) -> Result<GDObjectOpt, RustError> {
        let bytes: [u8; mem::size_of::<GDObjectOpt>()] = bytes
            .to_vec()
            .try_into()
            .map_err(|_| RustError::from(ErrorType::ObjectDeserialization))?;

        // SAFETY:
        // the bytes of the object are always validated on the server side
        // the server can never hold an invalid object, therefore the client can never
        // deserialise an invalid object
        Ok(unsafe { mem::transmute(bytes) })
    }

    pub fn debug(&self) -> String {
        format!("{:#?}", self)
    }
    pub fn lmao_clone(&self) -> Self {
        self.clone()
    }

    pub fn get_chunk_coord(&self) -> ChunkCoord {
        ChunkCoord::get_from_pos(self.x, self.y)
    }

    #[wasm_bindgen]
    pub fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
    #[wasm_bindgen]
    pub fn scale(&mut self, exp: i8) {
        self.x_scale_exp += exp;
        self.y_scale_exp += exp;
    }

    #[wasm_bindgen]
    pub fn rotate(&mut self, angle_5: i8) {
        self.x_angle += angle_5;
        self.y_angle += angle_5;
    }
}

pub fn convert_opt_transform(
    x_scale_exp: i8,
    x_angle: i8,
    y_scale_exp: i8,
    y_angle: i8,
) -> [f32; 4] {
    let x_scale = 2.0f32.powf((x_scale_exp as f32) / 12.0);
    let x_angle = (x_angle as f32 * 5.0).to_radians();

    let y_scale = 2.0f32.powf((y_scale_exp as f32) / 12.0);
    let y_angle = (y_angle as f32 * 5.0).to_radians();

    [
        x_scale * x_angle.cos(),
        x_scale * x_angle.sin(),
        y_scale * y_angle.cos(),
        y_scale * y_angle.sin(),
    ]
}

#[wasm_bindgen(js_name = "convert_opt_transform")]
pub fn wasm_convert_opt_transform(
    x_scale_exp: i8,
    x_angle: i8,
    y_scale_exp: i8,
    y_angle: i8,
) -> Vec<f32> {
    convert_opt_transform(x_scale_exp, x_angle, y_scale_exp, y_angle).into()
}
