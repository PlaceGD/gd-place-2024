use rust_shared::sprite::SpriteInfo;

use crate::utilgen::{DETAIL_SPRITES, MAIN_SPRITES};

pub fn get_max_bounding_box(id: u32) -> Option<(f32, f32)> {
    let mut rect_size = MAIN_SPRITES[id as usize].map(SpriteInfo::offset_rect_size);

    if let Some((w, h)) = DETAIL_SPRITES[id as usize].map(SpriteInfo::offset_rect_size) {
        if let Some((rw, rh)) = &mut rect_size {
            *rw = rw.max(w);
            *rh = rh.max(h);
        } else {
            rect_size = Some((w, h))
        }
    }

    rect_size
}
