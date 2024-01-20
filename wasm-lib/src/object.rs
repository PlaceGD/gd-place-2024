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

use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct GDColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub opacity: u8,
    pub blending: bool,
}

impl ToString for GDColor {
    fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}",
            self.r, self.g, self.b, self.opacity, self.blending as u8
        )
    }
}

impl FromStr for GDColor {
    type Err = ErrorType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s
            .split('|')
            .map(|s| s.parse::<u8>())
            .collect::<Result<Vec<u8>, ParseIntError>>()
            .map_err(|_| ErrorType::InvalidObjectString("color"))?;

        Ok(Self {
            r: split[0],
            g: split[1],
            b: split[2],
            opacity: split[3],
            blending: split[4] != 0,
        })
    }
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

macro_rules! gd_object {
    ($($prop:ident : $type:ty),*) => {
        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        #[wasm_bindgen]
        pub struct GDObject {
           $(pub $prop: $type),*
        }

        #[wasm_bindgen]
        impl GDObject {
            #[allow(clippy::too_many_arguments)]
            #[wasm_bindgen(constructor)]
            pub fn new(
                $($prop: $type),*
            ) -> Self {
                Self {
                    $($prop),*
                }
            }
            #[wasm_bindgen]
            pub fn serialize(&self) -> Result<String, RustError> {
                let mut out = String::new();
                $(
                    out.push_str(&self.$prop.to_string());
                    out.push(',');
                )*
                out.pop();
                Ok(out)
            }

            #[wasm_bindgen]
            pub fn deserialize(s: &str) -> Result<GDObject, RustError> {
                let mut split = s.split(',');
                Ok(Self {
                    $($prop: split.next().ok_or(ErrorType::InvalidObjectString("prop number of props :)"))?.parse().map_err(|_| ErrorType::InvalidObjectString("parse error"))?),*
                })
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
    };
}

gd_object! {
    id: u16,
    x: f32,
    y: f32,
    rotation: f32,
    flip_x: bool,
    flip_y: bool,
    scale: f32,
    z_layer: ZLayer,
    z_order: i8,
    main_color: GDColor,
    detail_color: GDColor
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
