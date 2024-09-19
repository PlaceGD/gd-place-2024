use crate::gd::layer::ZLayer;
use binrw::{BinRead, BinResult, BinWrite};
use wasm_bindgen::prelude::*;

use super::level::ChunkCoord;

#[binrw::parser(reader, endian)]
fn bool_parse() -> BinResult<bool> {
    let r: u8 = <_>::read_options(reader, endian, ())?;
    Ok(r == 1)
}
#[binrw::writer(writer, endian)]
fn bool_write(map: &bool) -> BinResult<()> {
    (if *map { 1u8 } else { 0u8 }).write_options(writer, endian, ())?;
    Ok(())
}

// IF THIS IS EVER CHANGED MAKE SURE TO CHANGE THE TYPESCRIPT TYPE IN SHAREDLIB
#[derive(Debug, Clone, Copy, Default, BinRead, BinWrite)]
#[wasm_bindgen]
#[repr(C, packed)]
#[brw(little)]
pub struct GDColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub opacity: u8,

    #[br(parse_with = bool_parse)]
    #[bw(write_with = bool_write)]
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

#[derive(Debug, Clone, Copy, Default, BinRead, BinWrite)]
#[wasm_bindgen(js_name = "GDObjectUnopt")]
#[brw(little)]

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
        ChunkCoord::get_from_pos(self.x, self.y)
    }

    pub fn offset(self, offset: glam::Vec2) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
            ..self
        }
    }

    pub fn from_str(s: &str) -> Self {
        let props = s.split(',').collect::<Vec<_>>();
        GDObject {
            id: u16::from_str_radix(props[0], 16).unwrap(),
            x: props[1].parse().unwrap(),
            y: props[2].parse().unwrap(),
            ix: props[3].parse().unwrap(),
            iy: props[4].parse().unwrap(),
            jx: props[5].parse().unwrap(),
            jy: props[6].parse().unwrap(),
            z_layer: ZLayer::from_gd_num(i8::from_str_radix(props[7], 16).unwrap()),
            z_order: i8::from_str_radix(props[8], 16).unwrap(),
            main_color: GDColor {
                r: u8::from_str_radix(props[9], 16).unwrap(),
                g: u8::from_str_radix(props[10], 16).unwrap(),
                b: u8::from_str_radix(props[11], 16).unwrap(),
                opacity: u8::from_str_radix(props[12], 16).unwrap(),
                blending: u8::from_str_radix(props[13], 16).unwrap() != 0,
            },
            detail_color: GDColor {
                r: u8::from_str_radix(props[14], 16).unwrap(),
                g: u8::from_str_radix(props[15], 16).unwrap(),
                b: u8::from_str_radix(props[16], 16).unwrap(),
                opacity: u8::from_str_radix(props[17], 16).unwrap(),
                blending: u8::from_str_radix(props[18], 16).unwrap() != 0,
            },
        }
    }
}
