use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
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

impl SpriteInfo {
    pub fn offset_rect_size(self) -> (f32, f32) {
        (
            self.size.0 as f32 + (self.offset.0 * 2.0).abs(),
            self.size.1 as f32 + (self.offset.1 * 2.0).abs(),
        )
    }
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

    for &(i, _) in get_available_objects() {
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

pub(crate) fn color_bleed(img: &mut DynamicImage) {
    let mut fixed = HashSet::new();
    let mut pass_fixed = HashSet::new();

    for _ in 0..3 {
        for x in 1..(img.width() - 1) {
            for y in 1..(img.height() - 1) {
                if img.get_pixel(x, y).0[3] == 0 {
                    if fixed.contains(&(x, y)) {
                        continue;
                    }

                    let x = x as i32;
                    let y = y as i32;
                    let (mut total, mut trgb) = (0usize, (0f32, 0f32, 0f32));

                    for i in -1..=1 {
                        for j in -1..=1 {
                            if (i, j) == (0, 0) {
                                continue;
                            }
                            let (cx, cy) = ((x + i) as u32, (y + j) as u32);
                            let [r, g, b, a] = img.get_pixel(cx, cy).0;
                            if a != 0 || fixed.contains(&(cx, cy)) {
                                total += 1;
                                trgb.0 += r as f32;
                                trgb.1 += g as f32;
                                trgb.2 += b as f32;
                            }
                        }
                    }

                    if total > 0 {
                        img.put_pixel(
                            x as u32,
                            y as u32,
                            Rgba([
                                (trgb.0 / total as f32) as u8,
                                (trgb.1 / total as f32) as u8,
                                (trgb.2 / total as f32) as u8,
                                0,
                            ]),
                        );
                        pass_fixed.insert((x as u32, y as u32));
                    }
                }
            }
        }
        fixed.extend(pass_fixed.drain());
    }
}

pub(crate) fn make_get_main_sprite_fn(data: &SpritesheetData) -> String {
    let mut ongy = [None; 4600];
    for (&k, &v) in &data.main_sprites {
        ongy[k as usize] = Some(v);
    }

    format!(
        "
pub const MAIN_SPRITES: [SpriteInfo; 4600] = {ongy:?};
    "
    )

    //     format!(
    //         "
    // pub fn get_main_sprite(id: u32) -> Option<SpriteInfo> {{
    //     Some(match id {{
    //         {},
    //         _ => return None,
    //     }})
    // }}
    //     ",
    //         data.main_sprites
    //             .iter()
    //             .map(|(id, info)| { format!("{id} => {info:#?}") })
    //             .join(",")
    //     )
}

pub(crate) fn make_get_detail_sprite_fn(data: &SpritesheetData) -> String {
    let mut ongy = [None; 4600];
    for (&k, &v) in &data.detail_sprites {
        ongy[k as usize] = Some(v);
    }

    format!(
        "
pub const DETAIL_SPRITES: [SpriteInfo; 4600] = {ongy:?};
    "
    )

    //     format!(
    //         "
    // pub fn get_detail_sprite(id: u32) -> Option<SpriteInfo> {{
    //     Some(match id {{
    //         {},
    //         _ => return None,
    //     }})
    // }}
    //     ",
    //         data.detail_sprites
    //             .iter()
    //             .map(|(id, info)| { format!("{id} => {info:#?}") })
    //             .join(",")
    //     )
}
