use crate::{layer::ZLayerMap, object::GDObject};

use std::collections::{BTreeMap, HashMap};

pub const CHUNK_SIZE_BLOCKS: u32 = 20;
pub const CHUNK_SIZE_UNITS: u32 = CHUNK_SIZE_BLOCKS * 30;
pub type DbKeyType = [u8; 20];

#[derive(Debug, Default)]
pub struct ObjectList {
    // pub blending: HashMap<DbKeyType, GDObject>,
    pub objects: BTreeMap<DbKeyType, GDObject>,
}

#[derive(Debug, Default)]
pub struct Level {
    pub chunks: HashMap<(i32, i32), ZLayerMap<ObjectList>>,
}
