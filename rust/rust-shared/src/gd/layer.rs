use std::str::FromStr;

// use serde::{Deserialize, Serialize};
use binrw::{BinRead, BinWrite};
use wasm_bindgen::prelude::*;

// use crate::ErrorType;

macro_rules! z_layers {
    ($($name:ident,)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, BinRead, BinWrite)]
        #[wasm_bindgen]
        #[brw(little, repr = u8)]
        pub enum ZLayer {
            $(
                $name,
            )*
        }
        pub const Z_LAYERS: &[ZLayer] = &[$(ZLayer::$name,)*];

        #[wasm_bindgen]
        pub fn z_layer_name(v: ZLayer) -> String {
            match v {
                $(
                    ZLayer::$name => stringify!($name).into(),
                )*
            }
        }
    };
}

impl ToString for ZLayer {
    fn to_string(&self) -> String {
        Z_LAYERS
            .iter()
            .position(|x| x == self)
            .unwrap_or(0) // should never fail but just in case
            .to_string()
    }
}

// impl FromStr for ZLayer {
//     type Err = ErrorType;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let id = s
//             .parse::<usize>()
//             .map_err(|_| ErrorType::InvalidObjectString("leayer2"))?;

//         if id >= Z_LAYERS.len() {
//             return Err(ErrorType::InvalidObjectString("leayer"));
//         }
//         Ok(Z_LAYERS[id])
//     }
// }
impl Default for ZLayer {
    fn default() -> Self {
        Self::B3
    }
}

impl ZLayer {
    pub fn from_gd_num(num: i8) -> ZLayer {
        // println!("Sex p[enis {}", num);
        // BG, MG, B5, B4, B3, B2, B1, P, T1, T2, T2, T4, G, UI, Max
        match num {
            -5 => ZLayer::B5,
            -3 => ZLayer::B4,
            -1 => ZLayer::B3,
            1 => ZLayer::B2,
            3 => ZLayer::B1,
            5 => ZLayer::T1,
            7 => ZLayer::T2,
            9 => ZLayer::T3,
            11 => ZLayer::T4,
            _ => ZLayer::B1,
            // _ => panic!("i will eat your balls"),
        }
    }
}

z_layers! {
    B5,
    B4,
    B3,
    B2,
    B1,
    T1,
    T2,
    T3,
    T4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ZLayerMap<T>([T; Z_LAYERS.len()]);

impl<T> ZLayerMap<T> {
    pub fn new(arr: [T; Z_LAYERS.len()]) -> Self {
        Self(arr)
    }

    pub fn get(&self, layer: ZLayer) -> &T {
        &self.0[layer as usize]
    }
    pub fn get_mut(&mut self, layer: ZLayer) -> &mut T {
        &mut self.0[layer as usize]
    }
    pub fn iter(&self) -> impl Iterator<Item = (&T, ZLayer)> {
        self.0.iter().enumerate().map(|(i, v)| (v, Z_LAYERS[i]))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut T, ZLayer)> {
        self.0.iter_mut().enumerate().map(|(i, v)| (v, Z_LAYERS[i]))
    }
}
