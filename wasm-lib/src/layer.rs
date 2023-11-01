use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

macro_rules! z_layers {
    ($($name:ident: $snake:ident,)*) => {
        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        #[wasm_bindgen]
        pub enum ZLayer {
            $(
                $name,
            )*
        }
        const Z_LAYERS: &[ZLayer] = &[$(ZLayer::$name,)*];
    };
}

z_layers! {
    B4: b4,
    B3: b3,
    B2: b2,
    B1: b1,
    T1: t1,
    T2: t2,
    T3: t3,
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
