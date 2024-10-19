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

#[derive(Clone, Copy)]
struct DigitSetLabels {
    silly: bool,
    pretty: bool,
    classic: bool,
    famous: bool,
}

impl DigitSetLabels {
    const fn random() -> Self {
        Self {
            silly: false,
            pretty: false,
            classic: false,
            famous: false,
        }
    }

    fn compat(&self, other: Self) -> bool {
        (!self.silly || other.silly)
            && (!self.pretty || other.pretty)
            && (!self.classic || other.classic)
            && (!self.famous || other.famous)
    }

    fn silly() -> Self {
        Self {
            silly: true,
            ..Self::random()
        }
    }

    fn pretty() -> Self {
        Self {
            pretty: true,
            ..Self::random()
        }
    }

    fn classic() -> Self {
        Self {
            classic: true,
            ..Self::random()
        }
    }

    fn famous() -> Self {
        Self {
            famous: true,
            ..Self::random()
        }
    }
}

macro_rules! parse_countdown_files {
    ($($creator:literal ($creator_name:literal): [$($index:expr => weights($($w:expr),+) $($label:ident)*),*]),*$(,)?) => {

        pub const DIGIT_SETS: usize = [$($($index,)*)*].len();
        pub const DIGIT_FILES: usize = [$($creator),*].len();

        pub fn get_countdown_sets(parse_gmd_file: fn(&str) -> LevelParseResult) -> ([LevelParseResult; DIGIT_FILES], [DigitSetPtr; DIGIT_SETS]) {
            #![allow(unused_assignments)]
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

        pub fn get_set_weights(index: usize) -> [f64; 4] {
            const WEIGHTS: [[f64; 4]; DIGIT_SETS] = [$(
                $(
                    [$($w as f64,)*],
                )*
            )*];

            WEIGHTS[index]
        }

        fn get_set_labels(index: usize) -> DigitSetLabels {
            const LABELS: [DigitSetLabels; DIGIT_SETS] = [$(
                $(
                    DigitSetLabels {
                        $($label: true,)*
                        ..DigitSetLabels::random()
                    },
                )*
            )*];

            LABELS[index]
        }
    };
}

parse_countdown_files! {
    "spu7nix"           ("Spu7Nix"):            [0 => weights(3,   1,   1,   0.4) silly,  // 0
                                                 1 => weights(3,   3,   3,   0.5) silly,  // 1
                                                 2 => weights(0.5, 1,   0.5, 3  ) silly], // 2
    "viprin"            ("Viprin"):             [3 => weights(3,   5,   5,   1  ) famous classic], // 3
    "deffie"            ("Cometface"):          [3 => weights(4,   3,   2,   3  ) classic], // 4
    "flow"              ("Flow"):               [3 => weights(1,   4,   4,   0.5) silly], // 5
    "galva"             ("G4lvatron"):          [3 => weights(7,   3,   2,   1  ) pretty classic], // 6
    "fungi"             ("Fungifity"):          [3 => weights(2.5, 5,   5,   1  ) pretty], // 7
    "thomartin"         ("Thomartin"):          [3 => weights(3,   4,   5,   2  ) pretty], // 8
    "dreaminginsanity"  ("DreamingInsanity"):   [3 => weights(0.4, 1,   0.4, 2  ) silly], // 9
    "echonox"           ("Echonox"):            [3 => weights(4,   5,   5,   6  ) classic], // 10
    "taman"             ("TamaN"):              [3 => weights(4,   5,   3,   1  ) classic], // 11
    "srguillester"      ("SrGuillester"):       [3 => weights(5,   2,   3,   3  ) silly famous], // 12
    "bianox"            ("Bianox"):             [3 => weights(2,   2,   2,   6  ) classic], // 13
    "sirhadoken"        ("SirHadoken"):         [3 => weights(1,   3,   3,   0.5) classic], // 14
    "jenkins"           ("Jenkins"):            [3 => weights(4,   4,   4,   1  ) pretty], // 15
    "kingtony"          ("KINGTONY"):           [3 => weights(3,   3,   3,   3  ) silly], // 16
    "domi"              ("Dominus"):            [3 => weights(2,   4,   4,   1  ) pretty], // 17
    "jonathangd"        ("JonathanGD"):         [3 => weights(3,   4,   4,   1  ) classic famous], // 18
    "exyl"              ("Exyl"):               [3 => weights(2,   2,   2,   2  )], // 19
    "jeyzor"            ("Jeyzor"):             [3 => weights(2,   1,   2,   4  ) classic], // 20
    "vermillion"        ("Vermillion"):         [3 => weights(3,   3,   3,   3  ) classic], // 21
    "mels"              ("MelX0exe"):           [2 => weights(2,   4,   4,   2  ) silly,  // 22
                                                 3 => weights(3,   3,   2,   4  ) silly pretty], // 23
    "evw"               ("EricVanWilderman"):   [0 => weights(3,   3,   3,   3  ) silly famous], // 24
    "serp"              ("Serponge"):           [3 => weights(3,   4,   5,   2  ) famous classic], // 25
    "bli"               ("bli"):                [3 => weights(5,   5,   5,   3  ) pretty famous], // 26
    "grax"              ("Grax"):               [3 => weights(3,   5,   4,   2  ) pretty], // 27
    "krmal"             ("KrmaL"):              [3 => weights(4,   4,   4,   2  ) famous classic], // 28
    "davjt"             ("DavJT"):              [3 => weights(3,   3,   4,   1  ) famous], // 29
    "audieo"            ("AudieoVisual"):       [3 => weights(2,   2,   4,   5  ) pretty], // 30
    "doggie"            ("Doggie"):             [0 => weights(2,   3,   4,   3  ) silly famous], // 31
    "pocke"             ("Pocke"):              [3 => weights(4,   2,   2,   1  ) silly], // 32
    "subwoofer"         ("Subwoofer"):          [3 => weights(2,   3,   3,   4  ) silly], // 33
    "para"              ("para"):               [3 => weights(3,   3,   3,   2  ) pretty], // 34
    "ww"                ("WerewolfGD"):         [3 => weights(2,   3,   3,   5  ) pretty], // 35
    "kips"              ("Kips"):               [0 => weights(4,   3,   3,   2  ) classic pretty], // 36
    "motley"            ("Motleyorc"):          [0 => weights(3,   3,   3,   1.5) classic pretty], // 37
    "nasgubb"           ("Nasgubb"):            [3 => weights(5,   4,   3,   4  ) classic pretty], // 38
    "tchotchke"         ("Tchotchke"):          [3 => weights(4,   2,   2,   1.5)], // 39
    "dreaminginsanity2" ("DreamingInsanity"):   [3 => weights(0,   1.5, 1.5, 1  ) silly], // 40
    "yunhaseu"          ("YunHaSeu"):           [0 => weights(3,   3,   2,   3  ) silly famous], // 41
    "rafer"             ("Rafer"):              [3 => weights(2,   2,   2,   8  ) silly pretty classic], // 42
    "ilrell"            ("ILRELL"):             [0 => weights(2,   4,   4,   1  ) pretty], // 43
}

pub const TEST_SETS: Option<[usize; 4]> = None; //Some([12, 21, 3, 24]);

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

const WEIGHT_POWER: f64 = 0.8;

// runs in nexus gen
pub fn generate_set_switches(n: usize) -> Vec<[usize; 4]> {
    let mut switches = vec![[0, 5, 9, 2]]; // final sets

    for i in 0..n {
        // choose 4 distinct sets (0..DIGIT_SETS) that are not in prev
        let mut sets = [usize::MAX; 4];
        let mut names = [""; 4];

        let prev = &switches[i];
        //let prev2 = &switches[i - 1];
        // let mut j = 0;
        // let mut r = 0;
        // while j < 4 {
        //     let random = DeterministicRandom(i + 3, r).random();
        //     let set = (random * DIGIT_SETS as f64) as usize;
        //     let name = get_creator_name(set);
        //     if !prev.contains(&set) && !sets.contains(&set) && !names.contains(&name) {
        //         sets[j] = set;
        //         names[j] = name;
        //         j += 1;
        //     }
        //     r += 1;
        // }

        let mut config = [DigitSetLabels::random(); 4];

        // insert one silly or one classic set
        if DeterministicRandom(i + 234, 0).random() < 0.5 {
            config[(DeterministicRandom(i + 535, 0).random() * 3.0) as usize] =
                DigitSetLabels::silly();
        } else {
            config[(DeterministicRandom(i + 535, 0).random() * 3.0) as usize] =
                DigitSetLabels::classic();
        }

        // insert one pretty set
        config[(DeterministicRandom(i + 234, 1).random() * 3.0) as usize] =
            DigitSetLabels::pretty();

        // insert one famous set
        config[(DeterministicRandom(i + 432, 2).random() * 3.0) as usize] =
            DigitSetLabels::famous();

        for j in 0..4 {
            let mut possible_sets = (0..DIGIT_SETS).collect::<Vec<_>>();
            possible_sets.retain(|&set| {
                config[j].compat(get_set_labels(set))
                    && !prev.contains(&set)
                    //&& !prev2.contains(&set)
                    && !sets.contains(&set)
                    && !names.contains(&get_creator_name(set))
            });

            if possible_sets.is_empty() {
                // ignore labels
                possible_sets = (0..DIGIT_SETS).collect::<Vec<_>>();
                possible_sets.retain(|&set| {
                    !prev.contains(&set)
                        //&& !prev2.contains(&set)
                        && !sets.contains(&set)
                        && !names.contains(&get_creator_name(set))
                });
            }

            // sort by weight
            possible_sets.sort_by(|&a, &b| {
                get_set_weights(a)[j]
                    .partial_cmp(&get_set_weights(b)[j])
                    .unwrap()
            });

            let total_weight = possible_sets
                .iter()
                .map(|&set| get_set_weights(set)[j].powf(WEIGHT_POWER))
                .sum::<f64>();

            let random = DeterministicRandom(i + 88, j + 6).random() * total_weight;

            let mut weight_sum = 0.0;
            for &set in &possible_sets {
                weight_sum += get_set_weights(set)[j].powf(WEIGHT_POWER);
                if weight_sum >= random {
                    sets[j] = set;
                    names[j] = get_creator_name(set);
                    break;
                }
            }
        }

        switches.push(sets);
    }

    {
        // check if proportions are correct
        let mut counts = [[0; 4]; DIGIT_SETS];

        let mut creator_counts = HashMap::new();

        for sets in &switches {
            for (i, &set) in sets.iter().enumerate() {
                counts[set][i] += 1;
                *creator_counts.entry(get_creator_name(set)).or_insert(0) += 1;
            }
        }

        for (i, &set) in counts.iter().enumerate() {
            println!(
                "{}: {:?}",
                get_creator_name(i),
                set.map(|x| x as f64 / n as f64)
            );
        }

        let mut creator_counts = creator_counts.into_iter().collect::<Vec<_>>();

        creator_counts.sort_by(|a, b| b.1.cmp(&a.1));

        for (name, count) in creator_counts {
            println!(
                "{}: {}% of the time",
                name,
                (count as f64 / n as f64) * 100.0
            );
        }
    }

    switches
}
