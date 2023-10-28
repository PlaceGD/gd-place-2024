use std::mem;

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
    pub fn serialize(&self) -> String {
        unsafe {
            let data =
                mem::transmute::<_, [u8; mem::size_of::<Self>()]>(std::ptr::read(self as *const _));

            String::from_utf8_unchecked(data.into())
        }
    }
    // #[wasm_bindgen]
    // pub fn deserialize(s: &[u8]) -> Self {
    //     todo!()
    //     // unsafe { mem::transmute::<_, _>(std::ptr::read(s.as_bytes())) }
    // }
}
