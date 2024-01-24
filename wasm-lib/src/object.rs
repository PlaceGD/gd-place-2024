use std::{mem, ptr};

use nalgebra::{matrix, vector};
// use bytemuck::{bytes_of, Pod, Zeroable};
use wasm_bindgen::prelude::*;

use crate::{
    layer::ZLayer,
    level::{ChunkCoord, CHUNK_SIZE_UNITS},
    util::get_chunk_coord,
    ErrorType, RustError,
};

use crate::log;

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
#[wasm_bindgen]
#[repr(C, packed)]
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

#[wasm_bindgen]
impl GDObject {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: u16,
        x: f32,
        y: f32,
        ix: f32,
        iy: f32,
        jx: f32,
        jy: f32,
        z_layer: ZLayer,
        z_order: i8,
        main_color: GDColor,
        detail_color: GDColor,
    ) -> Self {
        Self {
            id,
            x,
            y,
            ix,
            iy,
            jx,
            jy,
            z_layer,
            z_order,
            main_color,
            detail_color,
        }
    }

    pub fn bytes(&self) -> js_sys::Uint8Array {
        unsafe {
            let bytes: [u8; mem::size_of::<GDObject>()] =
                mem::transmute(ptr::read(self as *const _));

            js_sys::Uint8Array::view(&bytes)
        }
    }

    pub fn from_bytes(bytes: js_sys::Uint8Array) -> Result<GDObject, RustError> {
        let bytes: [u8; mem::size_of::<GDObject>()] = bytes
            .to_vec()
            .try_into()
            .map_err(|_| RustError::from(ErrorType::ObjectDeserialization))?;

        Ok(unsafe { mem::transmute(bytes) })
    }

    #[wasm_bindgen]
    pub fn debug(&self) -> String {
        format!("{:#?}", self)
    }

    #[wasm_bindgen]
    pub fn get_chunk_coord(&self) -> ChunkCoord {
        get_chunk_coord(self.x, self.y)
    }

    #[wasm_bindgen]
    pub fn apply_matrix(&mut self, a: f32, b: f32, c: f32, d: f32) {
        let mat = matrix![a, b; c, d];
        let i = mat * vector![self.ix, self.iy];
        let j = mat * vector![self.jx, self.jy];
        (self.ix, self.iy) = (i.x, i.y);
        (self.jx, self.jy) = (j.x, j.y);
    }

    #[wasm_bindgen]
    pub fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
    #[wasm_bindgen]
    pub fn scale(&mut self, factor: f32) {
        self.apply_matrix(factor, 0.0, 0.0, factor);
    }
    #[wasm_bindgen]
    pub fn skew(&mut self, x: f32, y: f32) {
        self.apply_matrix(1.0, x, y, 1.0);
    }

    #[wasm_bindgen]
    pub fn rotate(&mut self, angle: f32) {
        let angle = -angle.to_radians();
        let c = angle.cos();
        let s = angle.sin();
        self.apply_matrix(c, -s, s, c);
    }

    #[wasm_bindgen]
    pub fn x_basis_len(&self) -> f32 {
        vector![self.ix, self.iy].magnitude()
    }
    #[wasm_bindgen]
    pub fn y_basis_len(&self) -> f32 {
        vector![self.jx, self.jy].magnitude()
    }
}
