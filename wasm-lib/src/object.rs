// use bytemuck::{bytes_of, Pod, Zeroable};
use wasm_bindgen::prelude::*;

use crate::{
    layer::ZLayer,
    level::{ChunkCoord, CHUNK_SIZE_UNITS},
    util::get_chunk_coord,
    ErrorType, RustError,
};
use serde::{Deserialize, Serialize};

use crate::log;

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
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

struct Gaga(u64);

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
        rotation: i32,
        flip_x: bool,
        flip_y: bool,
        scale: f32,
        z_layer: ZLayer,
        z_order: i8,
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
            z_layer,
            z_order,
            main_color,
            detail_color,
        }
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> Result<String, RustError> {
        the_crazy_base::encode(unsafe { std::mem::transmute(std::ptr::read(self as *const _)) })
            .ok_or(ErrorType::ObjectSerialization.into())

        // let encoded: Vec<u8> = bincode::serialize(self).ok()?;

        // Some(general_purpose::URL_SAFE.encode(encoded))
    }
    #[wasm_bindgen]
    pub fn deserialize(s: String) -> Result<GDObject, RustError> {
        let decoded: [u8; 32] =
            the_crazy_base::decode(&s).ok_or::<RustError>(ErrorType::InvalidObjectString.into())?;

        Ok(unsafe { std::mem::transmute(decoded) })

        // let decoded = general_purpose::URL_SAFE.decode(&s).ok()?;

        // let b85 = base85::encode(&decoded);
        // let a85 = ascii85::encode(&decoded);
        // let diming = dremig::fuck(&decoded);

        // let out = bincode::deserialize(&decoded).ok()?;

        // let mine = halal::encode(unsafe { std::mem::transmute::<_, _>(out) });
        // log!(
        //     "current: {}, base85: {}, ascii85: {}, mine: {}, diming: {}",
        //     s.len(),
        //     b85.len(),
        //     a85.len(),
        //     mine.len(),
        //     diming.len()
        // );

        // Some(out)
    }

    #[wasm_bindgen]
    pub fn debug(&self) -> String {
        format!("{:#?}", self)
    }

    #[wasm_bindgen]
    pub fn get_chunk_coord(&self) -> ChunkCoord {
        get_chunk_coord(self.x, self.y)
    }

    // #[wasm_bindgen]
    // pub fn deserialize(s: &[u8]) -> Result<GDObject, i32> {
    //     let data = general_purpose::URL_SAFE
    //         .decode(s)
    //         .map_err(|_| DecodeError)?;

    //     unsafe { Ok(mem::transmute::<_, _>(&data)) }
    // }
}

mod the_crazy_base {

    const BASE: u128 = 126;

    fn convert_to_base(mut num: u128) -> Vec<u8> {
        let mut result = Vec::new();

        if num == 0 {
            return vec![0];
        }

        while num > 0 {
            let digit = (num % BASE) as u8;
            result.insert(0, digit);
            num /= BASE;
        }

        result
    }
    fn convert_from_base(num: &[u8]) -> u128 {
        if num.is_empty() {
            return 0;
        }

        num.iter()
            .rev()
            .enumerate()
            .map(|(i, &d)| (d as u128) * (BASE).pow(i as u32))
            .sum()
    }

    pub fn encode(b: [u8; 32]) -> Option<String> {
        let b = unsafe { std::mem::transmute::<_, [u128; 2]>(b) };

        let mut out = vec![];
        for b in b {
            let s = convert_to_base(b);

            out.extend(&s);
            if s.len() < convert_to_base(u128::MAX).len() {
                out.push(126)
            }
        }

        String::from_utf8(out).ok()
    }

    pub fn decode(s: &str) -> Option<[u8; 32]> {
        let mut ns = vec![];
        let max = convert_to_base(u128::MAX).len();

        for i in s.split('~') {
            for i in i.as_bytes().chunks(max) {
                ns.push(convert_from_base(i));
            }
        }
        let lol: [u128; 2] = ns.try_into().ok()?;
        Some(unsafe { std::mem::transmute(lol) })
    }
}
