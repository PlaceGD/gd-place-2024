use std::mem;

// use bytemuck::{bytes_of, Pod, Zeroable};
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use crate::log;

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct GDColor {
    r: u8,
    g: u8,
    b: u8,
    opacity: u8,
    blending: bool,
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
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct GDObject {
    pub id: u16,
    pub x: f32,
    pub y: f32,
    pub rotation: i32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub scale: f32,
    pub z_order: i32,
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
        rotation: i32,
        flip_x: bool,
        flip_y: bool,
        scale: f32,
        z_order: i32,
        main_color: GDColor,
        detail_color: GDColor,
    ) -> Self {
        Self {
            id,
            x,
            y,
            rotation,
            flip_x,
            flip_y,
            scale,
            z_order,
            main_color,
            detail_color,
        }
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> Option<String> {
        let encoded: Vec<u8> = bincode::serialize(self).ok()?;

        Some(general_purpose::URL_SAFE.encode(encoded))
    }
    #[wasm_bindgen]
    pub fn deserialize(s: String) -> Option<GDObject> {
        let decoded = general_purpose::URL_SAFE.decode(s).ok()?;

        bincode::deserialize(&decoded).ok()
    }

    #[wasm_bindgen]
    pub fn debug(&self) -> String {
        format!("{:#?}", self)
    }

    // #[wasm_bindgen]
    // pub fn deserialize(s: &[u8]) -> Result<GDObject, i32> {
    //     let data = general_purpose::URL_SAFE
    //         .decode(s)
    //         .map_err(|_| DecodeError)?;

    //     unsafe { Ok(mem::transmute::<_, _>(&data)) }
    // }
}
