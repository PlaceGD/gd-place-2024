use std::collections::HashMap;

use binrw::binrw;

use crate::gd::object::{GDColor, GDObject};

pub const DIGIT_WIDTH: f32 = 30.0 * 10.0;
pub const DIGIT_HEIGHT: f32 = 30.0 * 15.0;

pub struct LevelParseResult {
    pub objects: Vec<HashMap<u16, String>>,
    pub colors: HashMap<u16, GDColor>,
}

#[derive(Clone, Copy)]

pub struct DigitSetPtr {
    pub file: usize,
    pub set: usize,
}

macro_rules! parse_countdown_files {
    ($($creator:literal: [$($index:expr),*]),*$(,)?) => {

        pub const DIGIT_SETS: usize = [$($($index,)*)*].len();
        pub const DIGIT_FILES: usize = [$($creator),*].len();

        pub fn get_countdown_sets(parse_gmd_file: fn(&str) -> LevelParseResult) -> ([LevelParseResult; DIGIT_FILES], [DigitSetPtr; DIGIT_SETS]) {
            let files = [$({
                let mut parsed = parse_gmd_file(include_str!(concat!("countdowndigits/countdowndigits_", $creator, ".gmd")));
                parsed
                    .objects
                    .retain(|o| o.get(&20).map(|v| v.parse::<u16>().unwrap() >= 8) == Some(true));
                parsed
            }),*];
            let mut sets = [DigitSetPtr { file: 0, set: 0 }; DIGIT_SETS];

            let mut set_index = 0;
            let mut file_index = 0;

            $(
                $(
                    sets[set_index] = DigitSetPtr { file: file_index, set: $index };
                    set_index += 1;
                )*
                file_index += 1;
            )*

            (files, sets)
        }
    };
}

parse_countdown_files! {
    "spu7nix": [0, 1, 2],
    "viprin": [3],
    "deffie": [3],
    "flow": [3],
    "galva": [3],
    "fungi": [3],
    "thomartin": [3],
    "dreaminginsanity": [3],
    "echonox": [3],
    "taman": [3],
    "srguillester": [3],
}
#[binrw]
#[brw(little)]
pub struct DigitObjects {
    #[bw(calc(objs.len() as u32))]
    count: u32,
    #[br(count = count)]
    pub objs: Vec<GDObject>,
}

#[binrw]
#[brw(little)]
pub struct DigitSet(pub [DigitObjects; 10]);

#[binrw]
#[brw(little)]
pub struct CountdownDigitSets(
    pub [DigitSet; DIGIT_SETS],
    pub DigitObjects,
    pub DigitObjects,
    pub DigitObjects,
); // days marker, hours colon, minutes colon
   // should probably be a struct maybe now
