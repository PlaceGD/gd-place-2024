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

pub fn to_wstr(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub fn LOWORD(val: u32) -> u16 {
    (val & 0xffff) as u16
}

pub fn HIWORD(val: u32) -> u16 {
    (val >> 16) as u16
}
