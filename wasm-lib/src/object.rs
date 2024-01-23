use std::{mem, ptr};

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
}

// mod the_crazy_base {
//     use crate::log;

//     const BASE: u128 = 126;

//     pub fn convert_to_base(mut num: u128) -> Vec<u8> {
//         let mut result = Vec::new();

//         if num == 0 {
//             return vec![0];
//         }

//         while num > 0 {
//             let digit = (num % BASE) as u8;
//             result.insert(0, digit);
//             num /= BASE;
//         }

//         result
//     }
//     pub fn convert_from_base(num: &[u8]) -> u128 {
//         if num.is_empty() {
//             return 0;
//         }

//         num.iter()
//             .rev()
//             .enumerate()
//             .map(|(i, &d)| (d as u128) * BASE.pow(i as u32))
//             .sum()
//     }

//     pub fn encode(b: [u8; 32]) -> Option<String> {
//         let b = unsafe { std::mem::transmute::<_, [u128; 2]>(b) };

//         let mut out = vec![];
//         for b in b {
//             let s = convert_to_base(b);

//             out.extend(&s);
//             if s.len() < convert_to_base(u128::MAX).len() {
//                 out.push(126)
//             }
//         }
//         log!("baga: {:?}", out);
//         String::from_utf8(out).ok()
//     }

//     pub fn decode(s: &str) -> Option<[u8; 32]> {
//         let mut ns = vec![];
//         let max = convert_to_base(u128::MAX).len();

//         for i in s.split('~') {
//             for i in i.as_bytes().chunks(max) {
//                 ns.push(convert_from_base(i));
//             }
//         }
//         let lol: [u128; 2] = ns.try_into().ok()?;
//         Some(unsafe { std::mem::transmute(lol) })
//     }
// }
