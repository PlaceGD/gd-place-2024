use std::hash::Hasher;
use std::sync::LazyLock;
use std::{collections::HashMap, hash::Hash};

use binrw::binrw;
use js_sys::Math::random;

use crate::{
    console_log,
    gd::object::{GDColor, GDObject},
};

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
    ($($creator:literal ($creator_name:literal): [$($index:expr),*]),*$(,)?) => {

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

        pub fn get_creator_name(index: usize) -> &'static str {
            const NAMES: [(usize, &str); DIGIT_SETS] = [$($(($index, $creator_name),)*)*];

            NAMES[index].1
        }
    };
}

parse_countdown_files! {
    "spu7nix" ("Spu7Nix"): [0, 1, 2],
    "viprin" ("ViPriN"): [3],
    "deffie" ("Cometface"): [3],
    "flow" ("Flow"): [3],
    "galva" ("G4lvatron"): [3],
    "fungi" ("Fungifity"): [3],
    "thomartin" ("Thomartin"): [3],
    "dreaminginsanity" ("DreamingInsanity"): [3],
    "echonox" ("Echonox"): [3],
    "taman" ("TamaN"): [3],
    "srguillester" ("SrGuillester"): [3],
    "bianox" ("Bianox"): [3],
    "sirhadoken" ("SirHadoken"): [3],
    "jenkins" ("Jenkins"): [3],
    "kingtony" ("KINGTONY"): [3],
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

#[derive(Hash)]
struct DeterministicRandom(usize, usize);

impl DeterministicRandom {
    fn random(&self) -> f64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();
        (hash as f64) / (u64::MAX as f64)
    }
}

pub static SET_SWITCHES: LazyLock<Vec<[usize; 4]>> = LazyLock::new(|| {
    let mut switches = vec![[0; 4]];

    for i in 0..255 {
        // choose 4 distinct sets (0..DIGIT_SETS) that are not in prev
        let mut sets = [0; 4];
        let mut j = 0;
        let mut r = 0;
        let prev = &switches[i];

        while j < 4 {
            let random = DeterministicRandom(i + 6, r).random();
            let set = (random * DIGIT_SETS as f64) as usize;
            if !prev.contains(&set) && !sets.contains(&set) {
                sets[j] = set;
                j += 1;
            }
            r += 1;
        }

        switches.push(sets);
    }

    switches
});

pub fn choose_random_sets(num: usize) -> [usize; 4] {
    //console_log!("{:?}", SET_SWITCHES);
    SET_SWITCHES[num]
}
