use std::ops::{Add, Div, Mul, Sub};

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
