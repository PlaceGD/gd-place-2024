use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use itertools::Itertools;
use rust_shared::util::is_fully_transparent;
use rust_shared::{gd::special_ids, sprite::SpriteInfo};
use serde::Serialize;
use texture_packer::{exporter::ImageExporter, importer::ImageImporter, TexturePacker};

use crate::objects::{list::AVAILABLE_OBJECTS, sfx::SFX_TRIGGER_SOUNDS};

use super::config::PACKER_CONFIG;

#[derive(Debug, Clone, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct SpritesheetData {
    main_sprites: HashMap<u16, SpriteInfo>,
    detail_sprites: HashMap<u16, SpriteInfo>,
    sfx_icons: HashMap<&'static str, SpriteInfo>,
}

pub fn make_spritesheet() -> (DynamicImage, SpritesheetData) {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum SpriteKey {
        Main(u16),
        Detail(u16),
        Sfx(&'static str),
    }

    let mut packer: TexturePacker<'_, DynamicImage, SpriteKey> =
        TexturePacker::new_skyline(PACKER_CONFIG);

    for &(i, _) in AVAILABLE_OBJECTS.iter() {
        let special_tex = match i {
            special_ids::BG_TRIGGER => Some("bg_trigger"),
            special_ids::GROUND_TRIGGER => Some("ground_trigger"),
            special_ids::GROUND_2_TRIGGER => Some("ground_2_trigger"),
            special_ids::ARROW_TRIGGER => Some("arrow_trigger"),
            special_ids::SFX_TRIGGER => Some("sfx_trigger"),
            special_ids::SONG_TRIGGER => Some("song_trigger"),
            _ => None,
        };
        if let Some(tex) = special_tex {
            packer
                .pack_own(
                    SpriteKey::Main(i),
                    ImageImporter::import_from_file(&PathBuf::from(format!(
                        "textures/special/{}.png",
                        tex
                    )))
                    .unwrap(),
                )
                .unwrap();
        } else {
            let main =
                ImageImporter::import_from_file(&PathBuf::from(format!("textures/main/{}.png", i)))
                    .unwrap();
            let detail = ImageImporter::import_from_file(&PathBuf::from(format!(
                "textures/detail/{}.png",
                i
            )))
            .unwrap();

            if !is_fully_transparent(&main) {
                packer.pack_own(SpriteKey::Main(i), main).unwrap();
            }
            if !is_fully_transparent(&detail) {
                packer.pack_own(SpriteKey::Detail(i), detail).unwrap();
            }
        }
    }
    for i in SFX_TRIGGER_SOUNDS {
        packer
            .pack_own(
                SpriteKey::Sfx(i),
                ImageImporter::import_from_file(&PathBuf::from(format!(
                    "../../public/assets/objects/sfx_icons/{}.png",
                    i
                )))
                .unwrap(),
            )
            .unwrap();
    }

    let sheet = ImageExporter::export(&packer, None).unwrap();

    let mut main = HashMap::new();
    let mut detail = HashMap::new();
    let mut sfx_icons = HashMap::new();

    for (&key, f) in packer.get_frames() {
        let sprite_info = SpriteInfo {
            pos: (f.frame.x, f.frame.y),
            size: (f.frame.w, f.frame.h),
            rotated: f.rotated,
            offset: (
                (f.source.x as f32 + f.frame.w as f32 / 2.0) - f.source.w as f32 / 2.0,
                (f.source.y as f32 + f.frame.h as f32 / 2.0) - f.source.h as f32 / 2.0,
            ),
        };

        match key {
            SpriteKey::Main(id) => {
                main.insert(id, sprite_info);
            }
            SpriteKey::Detail(id) => {
                detail.insert(id, sprite_info);
            }
            SpriteKey::Sfx(s) => {
                sfx_icons.insert(s, sprite_info);
            }
        }
    }

    let data = SpritesheetData {
        main_sprites: main,
        detail_sprites: detail,
        sfx_icons,
    };

    (sheet, data)
}

pub fn color_bleed(img: &mut DynamicImage) {
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

pub fn make_get_main_sprite_fn(data: &SpritesheetData) -> String {
    let mut ongy = [None; 4600];
    for (&k, &v) in &data.main_sprites {
        ongy[k as usize] = Some(v);
    }

    format!(
        "
pub const MAIN_SPRITES: [Option<SpriteInfo>; 4600] = {ongy:?};
    "
    )
}

pub fn make_get_detail_sprite_fn(data: &SpritesheetData) -> String {
    let mut ongy = [None; 4600];
    for (&k, &v) in &data.detail_sprites {
        ongy[k as usize] = Some(v);
    }

    format!(
        "
pub const DETAIL_SPRITES: [Option<SpriteInfo>; 4600] = {ongy:?};
    "
    )
}

pub fn make_get_sfx_icon_sprite_fn(data: &SpritesheetData) -> String {
    let ongy = SFX_TRIGGER_SOUNDS
        .iter()
        .map(|s| data.sfx_icons[s])
        .collect_vec();

    format!(
        "
pub const SFX_ICON_SPRITES: &[SpriteInfo] = &{ongy:?};
    "
    )
}
