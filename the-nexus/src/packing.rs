use std::{collections::HashMap, path::PathBuf};

use image::{DynamicImage, GenericImageView};
use itertools::Itertools;
use texture_packer::{exporter::ImageExporter, importer::ImageImporter, TexturePacker};

use crate::{config::PACKER_CONFIG, objects::list::get_available_objects};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpriteInfo {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub rotated: bool,
    pub offset: (f32, f32),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpritesheetData {
    main_sprites: HashMap<u32, SpriteInfo>,
    detail_sprites: HashMap<u32, SpriteInfo>,
}

fn is_fully_transparent(img: &DynamicImage) -> bool {
    for x in 0..img.width() {
        for y in 0..img.height() {
            if img.get_pixel(x, y).0[3] != 0 {
                return false;
            }
        }
    }
    true
}

pub(crate) fn make_spritesheet() -> (DynamicImage, SpritesheetData) {
    let mut packer: TexturePacker<'_, image::DynamicImage, (u32, bool)> =
        TexturePacker::new_skyline(PACKER_CONFIG);

    for (i, _) in get_available_objects() {
        let main =
            ImageImporter::import_from_file(&PathBuf::from(format!("textures/main/{}.png", i)))
                .unwrap();
        let detail =
            ImageImporter::import_from_file(&PathBuf::from(format!("textures/detail/{}.png", i)))
                .unwrap();

        if !is_fully_transparent(&main) {
            packer.pack_own((i, false), main).unwrap();
        }
        if !is_fully_transparent(&detail) {
            packer.pack_own((i, true), detail).unwrap();
        }
    }

    let sheet = ImageExporter::export(&packer).unwrap();

    let mut main = HashMap::new();
    let mut detail = HashMap::new();

    for (&(id, is_detail), f) in packer.get_frames() {
        let map = if is_detail { &mut detail } else { &mut main };

        map.insert(
            id,
            SpriteInfo {
                pos: (f.frame.x, f.frame.y),
                size: (f.frame.w, f.frame.h),
                rotated: f.rotated,
                offset: (
                    (f.source.x as f32 + f.frame.w as f32 / 2.0) - f.source.w as f32 / 2.0,
                    (f.source.y as f32 + f.frame.h as f32 / 2.0) - f.source.h as f32 / 2.0,
                ),
            },
        );
    }

    let data = SpritesheetData {
        main_sprites: main,
        detail_sprites: detail,
    };

    (sheet, data)
}

pub(crate) fn make_get_main_sprite_fn(data: &SpritesheetData) -> String {
    format!(
        "
pub fn get_main_sprite(id: u32) -> Option<SpriteInfo> {{
    Some(match id {{
        {},
        _ => return None,
    }})
}}
    ",
        data.main_sprites
            .iter()
            .map(|(id, info)| { format!("{id} => {info:#?}") })
            .join(",")
    )
}

pub(crate) fn make_get_detail_sprite_fn(data: &SpritesheetData) -> String {
    format!(
        "
pub fn get_detail_sprite(id: u32) -> Option<SpriteInfo> {{
    Some(match id {{
        {},
        _ => return None,
    }})
}}
    ",
        data.detail_sprites
            .iter()
            .map(|(id, info)| { format!("{id} => {info:#?}") })
            .join(",")
    )
}
