use std::str::FromStr;

// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::ErrorType;

macro_rules! z_layers {
    ($($name:ident,)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[wasm_bindgen]
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

impl FromStr for ZLayer {
    type Err = ErrorType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .parse::<usize>()
            .map_err(|_| ErrorType::InvalidObjectString("leayer2"))?;

        if id >= Z_LAYERS.len() {
            return Err(ErrorType::InvalidObjectString("leayer"));
        }
        Ok(Z_LAYERS[id])
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
