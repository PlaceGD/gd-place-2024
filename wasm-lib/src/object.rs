use std::{mem, ptr};

// use bytemuck::{bytes_of, Pod, Zeroable};
use wasm_bindgen::prelude::*;

use crate::{layer::ZLayer, level::ChunkCoord, log, util::get_chunk_coord, ErrorType, RustError};

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
#[repr(C, packed)]
pub struct GDColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub opacity: u8,
    pub blending: bool,
}

#[wasm_bindgen]
impl GDColor {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8, opacity: u8, blending: bool) -> Self {
        Self {
            r,
            g,
            b,
            opacity,
            blending,
        }
    }
    #[wasm_bindgen]
    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            opacity: 255,
            blending: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GDObject {
    pub id: u16,
    pub x: f32,
    pub y: f32,

    pub ix: f32,
    pub iy: f32,
    pub jx: f32,
    pub jy: f32,

    pub z_layer: ZLayer,
    pub z_order: i8,
    pub main_color: GDColor,
    pub detail_color: GDColor,
}

impl GDObject {
    pub fn get_chunk_coord(&self) -> ChunkCoord {
        get_chunk_coord(self.x, self.y)
    }
}

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
        )[..] else {
            unreachable!()
        };

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
    }
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
        unsafe {
            let bytes: [u8; mem::size_of::<GDObjectOpt>()] =
                mem::transmute(ptr::read(self as *const _));

            js_sys::Uint8Array::view(&bytes)
        }
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

    pub fn get_chunk_coord(&self) -> ChunkCoord {
        get_chunk_coord(self.x, self.y)
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

#[wasm_bindgen]
pub fn convert_opt_transform(
    x_scale_exp: i8,
    x_angle: i8,
    y_scale_exp: i8,
    y_angle: i8,
) -> Vec<f32> {
    let x_scale = 2.0f32.powf((x_scale_exp as f32) / 12.0);
    let x_angle = (x_angle as f32 * 5.0).to_radians();

    let y_scale = 2.0f32.powf((y_scale_exp as f32) / 12.0);
    let y_angle = (y_angle as f32 * 5.0).to_radians();

    vec![
        x_scale * x_angle.cos(),
        x_scale * x_angle.sin(),
        y_scale * y_angle.cos(),
        y_scale * y_angle.sin(),
    ]
}

// #[wasm_bindgen]
// impl GDObject {
//     // #[allow(clippy::too_many_arguments)]
//     // #[wasm_bindgen(constructor)]
//     // pub fn new(
//     //     id: u16,
//     //     x: f32,
//     //     y: f32,
//     //     transform: Transform,
//     //     z_layer: ZLayer,
//     //     z_order: i8,
//     //     main_color: GDColor,
//     //     detail_color: GDColor,
//     // ) -> Self {
//     //     Self {
//     //         id,
//     //         x,
//     //         y,
//     //         transform,
//     //         z_layer,
//     //         z_order,
//     //         main_color,
//     //         detail_color,
//     //     }
//     // }

//     pub fn bytes(&self) -> js_sys::Uint8Array {
//         unsafe {
//             let bytes: [u8; mem::size_of::<GDObject>()] =
//                 mem::transmute(ptr::read(self as *const _));

//             js_sys::Uint8Array::view(&bytes)
//         }
//     }

//     pub fn from_bytes(bytes: js_sys::Uint8Array) -> Result<GDObject, RustError> {
//         let bytes: [u8; mem::size_of::<GDObject>()] = bytes
//             .to_vec()
//             .try_into()
//             .map_err(|_| RustError::from(ErrorType::ObjectDeserialization))?;

//         Ok(unsafe { mem::transmute(bytes) })
//     }

//     #[wasm_bindgen]
//     pub fn debug(&self) -> String {
//         format!("{:#?}", self)
//     }

//     #[wasm_bindgen]
//     pub fn get_chunk_coord(&self) -> ChunkCoord {
//         get_chunk_coord(self.x, self.y)
//     }

//     #[wasm_bindgen]
//     pub fn apply_matrix(&mut self, a: f32, b: f32, c: f32, d: f32) {
//         let mat = matrix![a, b; c, d];
//         let i = mat * vector![self.ix, self.iy];
//         let j = mat * vector![self.jx, self.jy];
//         (self.ix, self.iy) = (i.x, i.y);
//         (self.jx, self.jy) = (j.x, j.y);
//     }

//     #[wasm_bindgen]
//     pub fn translate(&mut self, x: f32, y: f32) {
//         self.x += x;
//         self.y += y;
//     }
//     #[wasm_bindgen]
//     pub fn scale(&mut self, factor: f32) {
//         self.apply_matrix(factor, 0.0, 0.0, factor);
//     }
//     #[wasm_bindgen]
//     pub fn skew(&mut self, x: f32, y: f32) {
//         self.apply_matrix(1.0, x, y, 1.0);
//     }

//     #[wasm_bindgen]
//     pub fn rotate(&mut self, angle: f32) {
//         let angle = -angle.to_radians();
//         let c = angle.cos();
//         let s = angle.sin();
//         self.apply_matrix(c, -s, s, c);
//     }

//     #[wasm_bindgen]
//     pub fn x_basis_len(&self) -> f32 {
//         vector![self.ix, self.iy].magnitude()
//     }
//     #[wasm_bindgen]
//     pub fn y_basis_len(&self) -> f32 {
//         vector![self.jx, self.jy].magnitude()
//     }
//     #[wasm_bindgen]
//     pub fn x_basis_angle(&self) -> f32 {
//         self.iy.atan2(self.ix).to_degrees()
//     }
//     #[wasm_bindgen]
//     pub fn y_basis_angle(&self) -> f32 {
//         self.jy.atan2(self.jx).to_degrees()
//     }

//     #[wasm_bindgen]
//     pub fn set_x_scale(&mut self, v: f32) {
//         let v = vector![self.ix, self.iy].normalize() * v;
//         self.ix = v.x;
//         self.iy = v.y;
//     }
//     #[wasm_bindgen]
//     pub fn set_y_scale(&mut self, v: f32) {
//         let v = vector![self.jx, self.jy].normalize() * v;
//         self.jx = v.x;
//         self.jy = v.y;
//     }
//     #[wasm_bindgen]
//     pub fn set_x_angle(&mut self, angle: f32) {
//         let l = vector![self.ix, self.iy].magnitude();
//         self.ix = angle.to_radians().cos() * l;
//         self.iy = angle.to_radians().sin() * l;
//     }
//     #[wasm_bindgen]
//     pub fn set_y_angle(&mut self, angle: f32) {
//         let l = vector![self.jx, self.jy].magnitude();
//         self.jx = angle.to_radians().cos() * l;
//         self.jy = angle.to_radians().sin() * l;
//     }
// }
