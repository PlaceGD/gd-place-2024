#[macro_export]
macro_rules! lerp {
    ($a:expr, $b:expr, $t:expr) => {{
        let a = $a;
        let b = $b;
        let t = $t;
        a + (b - a) * t
    }};
}
#[macro_export]
macro_rules! map {
    ($v:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {{
        let v = $v;
        let a = $a;
        let b = $b;
        let c = $c;
        let d = $d;
        c + (d - c) * (v - a) / (b - a)
    }};
}

use the_nexus::packing::SpriteInfo;
use wasm_bindgen::prelude::*;

use crate::utilgen::{get_detail_sprite, get_main_sprite};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
pub fn __log(s: &str) {
    log(s)
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::util::__log(&format!($($arg)*))
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T> Rect<T> {
    pub const fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }
}

impl<T> Rect<T>
where
    T: PartialOrd<T> + std::ops::Add<T, Output = T> + Copy, // (_)_)=>=>==>>=|==/==<=||=>
{
    pub fn contains(&self, x: T, y: T) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
    pub fn contains_excl(&self, x: T, y: T) -> bool {
        x > self.x && x < self.x + self.w && y > self.y && y < self.y + self.h
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.x, other.y)
            || self.contains(other.x + other.w, other.y)
            || self.contains(other.x, other.y + other.h)
            || self.contains(other.x + other.w, other.y + other.h)
    }
    pub fn overlaps_excl(&self, other: &Self) -> bool {
        self.contains_excl(other.x, other.y)
            || self.contains_excl(other.x + other.w, other.y)
            || self.contains_excl(other.x, other.y + other.h)
            || self.contains_excl(other.x + other.w, other.y + other.h)
    }
}
impl<T> Rect<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    pub fn area(&self) -> T {
        self.w * self.h
    }
}
impl<T> Rect<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    pub fn corners(&self) -> [(T, T); 4] {
        [
            (self.x, self.y),
            (self.x + self.w, self.y),
            (self.x + self.w, self.y + self.h),
            (self.x, self.y + self.h),
        ]
    }
    pub fn sides(&self) -> [((T, T), (T, T)); 4] {
        let [a, b, c, d] = self.corners();

        [(a, b), (b, c), (c, d), (d, a)]
    }
    pub fn perimeter(&self) -> T {
        self.h + self.h + self.w + self.w
    }
}

pub fn now() -> f64 {
    js_sys::Date::now()
}

pub fn get_max_bounding_box(id: u32) -> Option<(f32, f32)> {
    let mut rect_size = get_main_sprite(id).map(SpriteInfo::offset_rect_size);

    if let Some((w, h)) = get_detail_sprite(id).map(SpriteInfo::offset_rect_size) {
        if let Some((rw, rh)) = &mut rect_size {
            *rw = rw.max(w);
            *rh = rh.max(h);
        } else {
            rect_size = Some((w, h))
        }
    }

    rect_size
}
