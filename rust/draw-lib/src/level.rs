use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
};

use indexmap::IndexMap;
use rust_shared::gd::{layer::Z_LAYERS, level::CHUNK_SIZE_UNITS, object::GDObject};

use crate::{render::rectdraw::OBJECT_MAIN_OVER_DETAIL, utilgen::OBJECT_INFO};

pub type DbKey = [u8; 20];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
// impl Ord for ChunkCoord {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.x.cmp(&other.x).then(self.y.cmp(&other.y))
//     }
// }
#[derive(Debug, Clone, Copy)]
pub enum ObjectDraw {
    Both,
    TopTexture,
    BottomTexture,
}

pub struct LevelLayer<K> {
    pub sheet_batches: [[BTreeMap<i8, IndexMap<K, (GDObject, ObjectDraw)>>; 2]; 5],
}
impl<K: Default> Default for LevelLayer<K> {
    fn default() -> Self {
        Self {
            sheet_batches: Default::default(),
        }
    }
}

pub struct LevelChunk<K> {
    pub layers: [LevelLayer<K>; Z_LAYERS.len() + 1],
    pub last_time_visible: f64,
}
impl<K: Default> LevelChunk<K> {
    pub fn new(now: f64) -> Self {
        Self {
            layers: Default::default(),
            last_time_visible: now,
        }
    }
}

pub trait ObjKey: PartialEq {
    fn random_num(&self, i: u8) -> f32;
}

impl ObjKey for [u8; 20] {
    #[inline]
    fn random_num(&self, i: u8) -> f32 {
        let mut hash = 14695981039346656037u64;
        for &b in self {
            hash = hash ^ (b as u64);
            hash = hash.wrapping_mul(1099511628211u64);
        }

        hash = hash ^ (i as u64);
        hash = hash.wrapping_mul(1099511628211u64);

        (hash % 1000000u64) as f32 / 1000000.0
    }
}

impl ObjKey for usize {
    #[inline]
    fn random_num(&self, i: u8) -> f32 {
        0.0 // xd
    }
}

#[derive(Default)]
pub struct Level<K> {
    pub chunks: BTreeMap<ChunkCoord, LevelChunk<K>>,
}

impl<K: Default + Hash + Eq + Copy> Level<K> {
    // pub fn test() -> &'static [bool] {
    //     todo!()
    // }
    pub fn add_object(
        &mut self,
        obj: GDObject,
        key: K,
        chunk_override: Option<ChunkCoord>,
        now: f64,
    ) {
        let chunk = chunk_override.unwrap_or(ChunkCoord::get_from_pos(obj.x, obj.y));
        let sheet_idx = OBJECT_INFO[obj.id as usize].sheet as usize;

        let chunk = self
            .chunks
            .entry(chunk)
            .or_insert_with(|| LevelChunk::new(now));
        let [blending_batch, normal_batch] =
            &mut chunk.layers[obj.z_layer as usize].sheet_batches[sheet_idx];

        let main_over_detail = OBJECT_MAIN_OVER_DETAIL[obj.id as usize];

        let (gaga_a, gaga_b) = if !main_over_detail {
            (obj.main_color.blending, obj.detail_color.blending)
        } else {
            (obj.detail_color.blending, obj.main_color.blending)
        };

        match (gaga_a, gaga_b) {
            (true, true) => {
                blending_batch
                    .entry(obj.z_order)
                    .or_default()
                    .insert(key, (obj, ObjectDraw::Both));
            }
            (true, false) => {
                blending_batch
                    .entry(obj.z_order)
                    .or_default()
                    .insert(key, (obj, ObjectDraw::TopTexture));
                normal_batch
                    .entry(obj.z_order)
                    .or_default()
                    .insert(key, (obj, ObjectDraw::BottomTexture));
            }
            (false, true) => {
                normal_batch
                    .entry(obj.z_order)
                    .or_default()
                    .insert(key, (obj, ObjectDraw::TopTexture));

                let [next_blending_batch, _] =
                    &mut chunk.layers[obj.z_layer as usize + 1].sheet_batches[sheet_idx];
                next_blending_batch
                    .entry(obj.z_order)
                    .or_default()
                    .insert(key, (obj, ObjectDraw::BottomTexture));
            }
            (false, false) => {
                normal_batch
                    .entry(obj.z_order)
                    .or_default()
                    .insert(key, (obj, ObjectDraw::Both));
            }
        }
    }
    pub fn remove_object(&mut self, key: K) -> Option<GDObject> {
        for c in self.chunks.values_mut() {
            for layer_idx in 0..(c.layers.len() - 1) {
                for [blending_sheet, normal_sheet] in c.layers[layer_idx].sheet_batches.iter_mut() {
                    for order_map in blending_sheet.values_mut() {
                        if let Some((obj, draw)) = order_map.shift_remove(&key) {
                            match draw {
                                ObjectDraw::Both => return Some(obj),
                                ObjectDraw::TopTexture => {
                                    for order_map in normal_sheet.values_mut() {
                                        if order_map.shift_remove(&key).is_some() {
                                            break;
                                        }
                                    }
                                    return Some(obj);
                                }
                                ObjectDraw::BottomTexture => panic!("The shouldnt happen"),
                            }
                        }
                    }
                    for order_map in normal_sheet.values_mut() {
                        if let Some((obj, draw)) = order_map.shift_remove(&key) {
                            match draw {
                                ObjectDraw::Both => return Some(obj),
                                ObjectDraw::TopTexture => {
                                    'out: for [blending_sheet, _] in
                                        c.layers[layer_idx + 1].sheet_batches.iter_mut()
                                    {
                                        for order_map in blending_sheet.values_mut() {
                                            if order_map.shift_remove(&key).is_some() {
                                                break 'out;
                                            }
                                        }
                                    }
                                    return Some(obj);
                                }
                                ObjectDraw::BottomTexture => panic!("The shouldnt happen"),
                            }
                        }
                    }
                }
            }
        }
        None
    }
    // pub fn modify_object<F: FnOnce(&mut GDObject)>(&mut self, key: K, cb: F) {
    //     if let Some(mut o) = self.remove_object(key) {
    //         cb(&mut o);
    //         self.add_object(o, key);
    //     }
    // }
    pub fn foreach_obj_in_chunk<F>(&self, chunk: ChunkCoord, mut f: F)
    where
        F: FnMut(K, &GDObject),
    {
        let mut visited = HashSet::new();
        if let Some(chunk) = self.chunks.get(&chunk) {
            for (key, (obj, _)) in chunk
                .layers
                .iter()
                .take(Z_LAYERS.len())
                .flat_map(|l| l.sheet_batches.iter())
                .flat_map(|s| s.iter())
                .flat_map(|m| m.iter())
                .flat_map(|(_, v)| v.iter())
            {
                if visited.insert(*key) {
                    f(*key, obj)
                }
            }
        }
    }
    pub fn get_obj_by_key(&self, key: K) -> Option<&GDObject> {
        for chunk in self.chunks.values() {
            for (_, m) in chunk
                .layers
                .iter()
                .take(Z_LAYERS.len())
                .flat_map(|l| l.sheet_batches.iter())
                .flat_map(|s| s.iter())
                .flat_map(|m| m.iter())
            {
                if let Some((o, _)) = m.get(&key) {
                    return Some(o);
                }
            }
        }
        None
    }
}

// pub type DbKey = [u8; 20];

// #[derive(Debug, Default)]
// pub struct ObjectList {
//     pub objects: BTreeMap<i8, IndexMap<DbKey, GDObject>>,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct ChunkCoord {
//     pub x: i32,
//     pub y: i32,
// }
// impl ChunkCoord {
//     pub fn get_from_pos(x: f32, y: f32) -> Self {
//         Self {
//             x: (x / CHUNK_SIZE_UNITS as f32).floor() as i32,
//             y: (y / CHUNK_SIZE_UNITS as f32).floor() as i32,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct ChunkInfo {
//     pub objects: ZLayerMap<ObjectList>,
//     pub last_time_visible: f64,
// }

// impl ChunkInfo {
//     pub fn new() -> Self {
//         Self {
//             last_time_visible: now(),
//             objects: Default::default(),
//         }
//     }
// }

// #[derive(Debug, Default)]
// pub struct Level {
//     pub chunks: HashMap<ChunkCoord, ChunkInfo>,
// }

// impl Level {
//     pub fn foreach_obj_in_z<F>(
//         &self,
//         layer: ZLayer,
//         z_order: i8,
//         mut f: F,
//         preview: Option<GDObject>,
//     ) where
//         F: FnMut(DbKey, &GDObject),
//     {
//         for (&coord, chunk) in &self.chunks {
//             if let Some(v) = chunk.objects.get(layer).objects.get(&z_order) {
//                 for (&key, obj) in v {
//                     f(key, obj)
//                 }
//             }
//             if let Some(o) = preview {
//                 if o.z_layer == layer && o.z_order == z_order && o.get_chunk_coord() == coord {
//                     f([0; 20], &o)
//                 }
//             }
//         }
//     }
//     pub fn foreach_obj_in_chunk<F>(&self, chunk: ChunkCoord, mut f: F)
//     where
//         F: FnMut(DbKey, &GDObject),
//     {
//         if let Some(chunk) = self.chunks.get(&chunk) {
//             for (&key, obj) in chunk
//                 .objects
//                 .iter()
//                 .flat_map(|(list, _)| list.objects.iter())
//                 .flat_map(|(_, map)| map.iter())
//             {
//                 f(key, obj)
//             }
//         }
//     }
//     pub fn get_obj_by_key(&self, key: DbKey) -> Option<&GDObject> {
//         for c in self.chunks.values() {
//             for (list, _) in c.objects.iter() {
//                 for m in list.objects.values() {
//                     if let Some(o) = m.get(&key) {
//                         return Some(o);
//                     }
//                 }
//             }
//         }
//         None
//     }
// }
