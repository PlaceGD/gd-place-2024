use crate::object::GDObject;

use std::collections::HashMap;

pub const CHUNK_SIZE: u32 = 20 * 30;

pub type ObjectList = [Vec<GDObject>; 7];

#[derive(Debug, Default)]
pub struct Level {
    chunks: HashMap<(i32, i32), ObjectList>,
}
