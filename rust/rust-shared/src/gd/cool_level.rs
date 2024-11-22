use std::collections::{BTreeMap, HashMap};

use indexmap::IndexMap;
use slab::Slab;

use crate::util::now;

use super::{layer::Z_LAYERS, object::GDObject};

pub type DbKey = [u8; 20];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}
impl ChunkCoord {
    pub fn get_from_pos(x: f32, y: f32) -> Self {
        Self {
            x: (x / CHUNK_SIZE_UNITS as f32).floor() as i32,
            y: (y / CHUNK_SIZE_UNITS as f32).floor() as i32,
        }
    }
}

pub struct LevelChunk {
    pub batches: [[[BTreeMap<i8, IndexMap<DbKey, GDObject>>; 2]; 5]; Z_LAYERS.len() + 1],
    pub last_time_visible: f64,
}
impl LevelChunk {
    pub fn new() -> Self {
        Self {
            batches: Default::default(),
            last_time_visible: now(),
        }
    }
}

struct CoolLevel {
    chunks: HashMap<ChunkCoord, LevelChunk>,
}

impl CoolLevel {
    pub fn add_object(&mut self, obj: GDObject, key: DbKey) {
        let chunk = obj.get_chunk_coord();

        let chunk = self.chunks.entry(chunk).or_insert_with(LevelChunk::new);
    }
}
