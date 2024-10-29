use image::{DynamicImage, GenericImageView};
use wasm_bindgen::prelude::*;

pub fn is_fully_transparent(img: &DynamicImage) -> bool {
    for x in 0..img.width() {
        for y in 0..img.height() {
            if img.get_pixel(x, y).0[3] != 0 {
                return false;
            }
        }
    }
    true
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h * 6.0;

    let c = v * s;
    let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());

    let (r, g, b) = if (0.0..1.0).contains(&h) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let m = v - c;
    let (r, g, b) = (r + m, g + m, b + m);

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

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

use glam::Vec2;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[doc(hidden)]
pub fn __log(s: &str) {
    log(s)
}

#[macro_export]
macro_rules! console_log {
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

// cant be arsed anymore
impl Rect<f32> {
    pub fn expanded(self, by: f32) -> Self {
        let center_x = self.x + self.w / 2.0;
        let center_y = self.y + self.h / 2.0;

        Self {
            x: center_x - self.w * by / 2.0,
            y: center_y - self.h * by / 2.0,
            w: self.w * by,
            h: self.h * by,
        }
    }
}

pub fn now() -> f64 {
    js_sys::Date::now()
}

pub fn random() -> f64 {
    js_sys::Math::random()
}

fn sign(p1: Vec2, p2: Vec2, p3: Vec2) -> f32 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}

pub fn point_in_triangle(pt: Vec2, v1: Vec2, v2: Vec2, v3: Vec2) -> bool {
    let (d1, d2, d3) = (sign(pt, v1, v2), sign(pt, v2, v3), sign(pt, v3, v1));

    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    !(has_neg && has_pos)
}

// pub fn quick_image_load(bytes: &[u8]) -> DynamicImage {
//     use image::io::Reader as ImageReader;
//     use std::io::Cursor;

//     ImageReader::new(Cursor::new(bytes))
//         .with_guessed_format()
//         .unwrap()
//         .decode()
//         .unwrap()
// }
