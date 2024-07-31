use std::collections::HashMap;

use itertools::Itertools;

pub type ObjectMap = HashMap<u16, String>;

pub fn parse_obj(v: impl AsRef<str>) -> HashMap<u16, String> {
    println!("lol {}", v.as_ref());
    v.as_ref()
        .split(',')
        .tuples()
        .map(|(k, v)| (k.parse().unwrap(), v.into()))
        .collect()
}
