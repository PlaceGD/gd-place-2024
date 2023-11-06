use crate::{
    layer::{ZLayer, ZLayerMap},
    object::GDObject,
    util::{now, Rect},
};

use indexmap::IndexMap;
use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};
use wasm_bindgen::prelude::*;

pub const CHUNK_SIZE_BLOCKS: u32 = 20;
pub const CHUNK_SIZE_UNITS: u32 = CHUNK_SIZE_BLOCKS * 30;

pub const LEVEL_WIDTH_BLOCKS: u32 = 400;
pub const LEVEL_HEIGHT_BLOCKS: u32 = 80;
pub const LEVEL_WIDTH_UNITS: u32 = LEVEL_WIDTH_BLOCKS * 30;
pub const LEVEL_HEIGHT_UNITS: u32 = LEVEL_HEIGHT_BLOCKS * 30;

pub const LEVEL_RECT_BLOCKS: Rect<i32> =
    Rect::new(0, 0, LEVEL_WIDTH_BLOCKS as i32, LEVEL_HEIGHT_BLOCKS as i32);
pub const LEVEL_RECT_UNITS: Rect<i32> =
    Rect::new(0, 0, LEVEL_WIDTH_UNITS as i32, LEVEL_HEIGHT_UNITS as i32);

pub type DbKey = [u8; 20];

#[derive(Debug, Default)]
pub struct ObjectList {
    pub objects: BTreeMap<i8, IndexMap<DbKey, GDObject>>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct ChunkInfo {
    pub objects: ZLayerMap<ObjectList>,
    pub last_time_visible: f64,
}

impl ChunkInfo {
    pub fn new() -> Self {
        Self {
            last_time_visible: now(),
            objects: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Level {
    pub chunks: HashMap<ChunkCoord, ChunkInfo>,
}

impl Level {
    pub fn foreach_obj_in<F>(&self, layer: ZLayer, z_order: i8, mut f: F)
    where
        F: FnMut(&GDObject),
    {
        for (&ChunkCoord { x, y }, chunk) in &self.chunks {
            if let Some(v) = chunk.objects.get(layer).objects.get(&z_order) {
                for (&key, obj) in v {
                    f(obj)
                }
            }
        }
    }
}
