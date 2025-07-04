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

#[derive(Clone)]
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

        fn get_set_by_creator(creator: &str) -> usize {
            const NAMES: [(usize, &str); DIGIT_SETS] = [$($(($index, $creator),)*)*];

            NAMES.iter().enumerate().find(|(_, &(_, name))| name == creator).expect(&format!("couldnt find creator: {}", creator)).0
        }
    };
}

parse_countdown_files! {
    "spu7nix"           ("Spu7Nix"):            [1 => weights(4,   3,   3,   1  ) pretty], // 0
    "viprin"            ("Viprin"):             [3 => weights(3,   5,   5,   1  ) famous classic], // 1
    "deffie"            ("Cometface"):          [3 => weights(4,   3,   2,   3  ) classic], // 2
    "flow"              ("Flow"):               [3 => weights(1,   1,   1,   0  ) ], // 3
    "galva"             ("G4lvatron"):          [3 => weights(7,   3,   2,   1  ) pretty classic], // 4
    "fungi"             ("Fungifity"):          [3 => weights(2.5, 5,   5,   1  ) pretty], // 5
    "thomartin"         ("Thomartin"):          [3 => weights(3,   4,   5,   2  ) pretty], // 6
    "dreaminginsanity3" ("DreamingInsanity"):   [3 => weights(2,   3,   3,   4  ) pretty], // 7
    "echonox"           ("Echonox"):            [3 => weights(4,   5,   5,   6  ) classic], // 8
    "taman"             ("TamaN"):              [3 => weights(4,   5,   3,   1  ) classic], // 9
    "srguillester"      ("SrGuillester"):       [3 => weights(5,   2,   3,   3  ) silly famous], // 10
    "bianox"            ("Bianox"):             [3 => weights(2,   2,   2,   6  ) classic], // 11
    "sirhadoken"        ("SirHadoken"):         [3 => weights(3,   3,   3,   3  ) classic], // 12
    "jenkins"           ("Jenkins"):            [3 => weights(4,   4,   4,   1  ) pretty], // 13
    "kingtony"          ("KINGTONY"):           [3 => weights(2,   2,   3,   3  )], // 14
    "domi"              ("Dominus"):            [3 => weights(2,   4,   4,   1  ) pretty], // 15
    "jonathangd"        ("JonathanGD"):         [3 => weights(2,   2,   2,   1  ) classic famous], // 16
    "exyl"              ("Exyl"):               [3 => weights(2,   2,   2,   2  )], // 17
    "jeyzor"            ("Jeyzor"):             [3 => weights(2,   1,   2,   4  ) classic], // 18
    "vermillion"        ("Vermillion"):         [3 => weights(3,   3,   3,   3  ) classic], // 19
    "mels"              ("MelX0exe"):           [2 => weights(2,   4,   4,   2  ) silly,  // 20
                                                 3 => weights(4,   3,   2,   4  ) pretty], // 21
    "evw"               ("EricVanWilderman"):   [0 => weights(3,   3,   3,   3  ) silly famous], // 22
    "serp"              ("Serponge"):           [3 => weights(3,   4,   5,   2  ) famous classic], // 23
    "bli"               ("bli"):                [3 => weights(6,   6,   6,   3  ) pretty famous], // 24
    "grax"              ("Grax"):               [3 => weights(3,   5,   4,   2  ) pretty], // 25
    "krmal"             ("KrmaL"):              [3 => weights(4,   4,   4,   2  ) famous classic], // 26
    "davjt"             ("DavJT"):              [3 => weights(2,   2,   3,   1  ) famous], // 27
    "audieo"            ("AudieoVisual"):       [3 => weights(2,   2,   4,   5  ) pretty], // 28
    "doggie"            ("Doggie"):             [0 => weights(2,   3,   4,   3  ) silly famous], // 29
    "pocke"             ("Pocke"):              [3 => weights(4,   2,   2,   1  ) silly], // 30
    "subwoofer"         ("Subwoofer"):          [3 => weights(2,   3,   3,   4  ) silly], // 31
    "para"              ("para"):               [3 => weights(3,   3,   3,   2  ) pretty], // 32
    "ww"                ("WerewolfGD"):         [3 => weights(2,   3,   3,   5  ) pretty], // 33
    "kips"              ("Kips"):               [0 => weights(4,   3,   3,   2  ) classic pretty], // 34
    "motley"            ("Motleyorc"):          [0 => weights(3,   3,   3,   1.5) classic pretty], // 35
    "nasgubb"           ("Nasgubb"):            [3 => weights(5,   4,   3,   4  ) classic pretty], // 36
    "tchotchke"         ("Tchotchke"):          [3 => weights(4,   2,   2,   1.5) pretty], // 37
    "dreaminginsanity2" ("DreamingInsanity"):   [3 => weights(0,   1.5, 1.5, 1  ) ], // 38
    "yunhaseu"          ("YunHaSeu"):           [0 => weights(3,   3,   2,   3  ) silly famous], // 39
    "rafer"             ("Rafer"):              [3 => weights(2,   2,   5,   9  ) silly pretty classic], // 40
    "ilrell"            ("ILRELL"):             [0 => weights(2,   4,   4,   1  ) pretty], // 41
    "culuc"             ("Culuc"):              [3 => weights(5,   3,   4,   4  ) famous pretty silly], // 42
    "boldstep"          ("BoldStep"):           [0 => weights(6,   3,   3,   2  )], // 43
    "evasium"           ("Evasium"):            [0 => weights(1,   1,   2,   4  ) classic], // 44
    "dorami"            ("Dorami"):             [0 => weights(3,   3,   3,   3  ) famous silly], // 45
    "npesta"            ("npesta"):             [0 => weights(3,   3,   3,   3  ) famous silly], // 46
    "partition"         ("Partition"):          [0 => weights(3,   4,   2,   3  ) famous pretty], // 47
    "vrymer"            ("vrymer"):             [3 => weights(3,   1,   1,   3  ) silly pretty], // 48
    "meeloz"            ("meeloz"):             [3 => weights(1,   2,   3,   4  ) silly], // 49
    "flow2"             ("Flow"):               [3 => weights(2,   3,   3,   2  ) pretty], // 50
    "glittershroom"     ("Glittershroom"):      [0 => weights(4,   3,   3,   4  ) classic], // 51
    "loco"              ("xloco"):              [3 => weights(5,   5,   4,   3  ) pretty], // 52
    "tech"              ("Technical"):          [0 => weights(1,   1,   1,   1  ) silly famous], // 53
    "connot"            ("connot"):             [3 => weights(4,   2,   3,   3  ) silly], // 54
    "rustam"            ("Rustam"):             [0 => weights(3,   3,   3,   3  ) classic], // 55
    "robtop"            ("RobTopGames"):        [0 => weights(8,   8,   8,   8  ) famous classic silly], // 56
    "desticy"           ("DesTicY"):            [0 => weights(5,   2,   4,   4  ) classic pretty], // 57
    "xeno"              ("xenoteric"):          [3 => weights(4,   3,   2,   5  ) pretty], // 58
    "logi"              ("logiking"):           [0 => weights(4,   3,   3,   3  ) pretty], // 59
    "aeonair"           ("AeonAir"):            [0 => weights(4,   2,   2,   5  ) silly classic famous], // 60
    "breadking"         ("Breadking"):          [3 => weights(2,   2,   2,   3  )], // 61
    "neige"             ("Neigefeu"):           [0 => weights(3,   3,   3,   3  ) classic pretty], // 62
    "juniper"           ("Juniper"):            [0 => weights(2,   3,   3,   3  ) famous silly], // 63
    "wulzy"             ("Wulzy"):              [0 => weights(3,   3,   3,   5  )], // 64
    "platnuu"           ("Platnuu"):            [0 => weights(4,   3,   3,   5  ) pretty], // 65
    "radiationv2"       ("RadiationV2"):        [3 => weights(5,   4,   4,   3  ) pretty], // 66
    "smiffy"            ("Smiffy777"):          [0 => weights(3,   3,   3,   3  ) silly], // 67
    "knots"             ("Knots"):              [3 => weights(4,   3,   3,   4  ) pretty], // 68
    "terron"            ("Terron"):             [0 => weights(4,   3,   3,   5  ) classic pretty], // 69
    "aqua"              ("Aquatias"):           [3 => weights(3,   2,   2,   4  ) classic], // 70
    "devon"             ("Thedevon"):           [0 => weights(4,   2,   2,   5  ) classic], // 71
    "digi"              ("Digitalight"):        [3 => weights(3,   3,   3,   3  ) pretty], // 72
    "chunlv1"           ("chunlv1"):            [3 => weights(3,   3,   3,   6  ) pretty], // 73
    "stormfly"          ("Stormfly"):           [0 => weights(1,   1,   1,   1  ) classic], // 74
    "verti"             ("verticallity"):       [3 => weights(4,   3,   5,   7  ) pretty], // 75
    "goose"             ("Goose"):              [0 => weights(6,   6,   6,   2  ) pretty classic], // 76
    "voxicat"           ("Voxicat"):            [3 => weights(5,   5,   5,   5  ) famous pretty], // 77
    "knobbel"           ("Knobbelboy"):         [0 => weights(4,   4,   4,   3  ) famous classic silly], // 78
    "immaxx"            ("ImMaxX1"):            [3 => weights(2,   3,   3,   3  ) pretty], // 79
    "dangerkat"         ("DangerKat"):          [3 => weights(3,   3,   3,   5  ) pretty], // 80
    "perox8"            ("Perox8"):             [3 => weights(1,   2,   2,   3  ) pretty], // 81
    "immaxx2"           ("ImMaxX"):             [3 => weights(3,   3,   3,   4  ) pretty silly], // 82
    "zoink"             ("Zoink"):              [0 => weights(0,   0,   3,   4  ) silly], // 83
    "sakura"            ("Sakura"):             [3 => weights(4,   3,   3,   2  ) pretty], // 84
}

pub const TEST_SETS: Option<[usize; 4]> = None; //Some([0, 68, 82, 84]);

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
    pub [DigitObjects; 6],
    pub [DigitObjects; 6],
); // days marker, hours colon, minutes colon
   // should probably be a struct maybe now

const WEIGHT_POWER: f64 = 0.8;

// runs in nexus gen
#[cfg(not(target_arch = "wasm32"))]
pub fn generate_set_switches(n: usize) -> Vec<[usize; 4]> {
    use rand::prelude::*;
    let mut switches = vec![None; n];

    switches[0] = Some([1, 50, 0, 7]); // final sets
    switches[1] = Some([1, 56, 69, 66]);
    switches[2] = Some([1, 78, 60, 46]);
    switches[3] = Some([1, 77, 24, 68]);
    switches[4] = Some([1, 76, 0, 4]);
    switches[5] = Some([1, 42, 59, 75]);

    let set_switch_minutes = 20;
    let mut release_days = HashMap::new();

    let mut set_day_end = |day: usize, sets: [&str; 4]| {
        let index = day * 60 * 24 / set_switch_minutes;
        switches[index] = Some(sets.map(|s| get_set_by_creator(s)));
        for set in sets {
            release_days.insert(get_set_by_creator(set), day);
        }
    };

    // let mut set_release_day = |day: usize, sets: &[&str]| {
    //     for set in sets {
    //         release_days.insert(get_set_by_creator(set), day);
    //     }
    // };

    set_day_end(7, ["serp", "desticy", "kips", "audieo"]);
    set_day_end(6, ["goose", "culuc", "knots", "nasgubb"]);
    set_day_end(5, ["krmal", "partition", "galva", "rafer"]);
    set_day_end(4, ["yunhaseu", "dorami", "juniper", "smiffy"]);
    set_day_end(3, ["bli", "voxicat", "loco", "echonox"]); // change to xender?
    set_day_end(2, ["srguillester", "doggie", "aeonair", "zoink"]);
    set_day_end(1, ["knobbel", "robtop", "evw", "npesta"]);

    let get_release_day = |index| release_days.get(&index).copied().unwrap_or(9);

    let mut rng = StdRng::seed_from_u64(42);

    for i in 0..(n - 1) {
        if switches[i + 1].is_some() {
            continue;
        }
        // choose 4 distinct sets (0..DIGIT_SETS) that are not in prev
        let mut sets = [usize::MAX; 4];
        let mut names = [""; 4];

        let prev = &switches[i].unwrap();
        let next = &switches.get(i + 2).cloned().flatten();
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

        let day = i * set_switch_minutes / (60 * 24);

        let mut config = [DigitSetLabels::random(); 4];

        println!("{}", rng.gen::<f64>());

        let sillytime = rng.gen::<f64>() < 0.1;

        if sillytime {
            config = [DigitSetLabels::silly(); 4]; // wacky time silly tinnme
        } else {
            // insert one silly or one classic set
            if rng.gen::<f64>() < 0.5 {
                config[(rng.gen::<f64>() * 3.0f64) as usize] = DigitSetLabels::silly();
            } else {
                config[(rng.gen::<f64>() * 3.0f64) as usize] = DigitSetLabels::classic();
            }

            // insert one pretty set
            config[(rng.gen::<f64>() * 3.0f64) as usize] = DigitSetLabels::pretty();

            // insert one famous set
            config[(rng.gen::<f64>() * 3.0f64) as usize] = DigitSetLabels::famous();
        }

        for j in 0..4 {
            let mut possible_sets = (0..DIGIT_SETS).collect::<Vec<_>>();
            possible_sets.retain(|&set| {
                config[j].compat(get_set_labels(set))
                    && !prev.contains(&set)
                    && next.map_or(true, |next| !next.contains(&set))
                    //&& !prev2.contains(&set)
                    && !sets.contains(&set)
                    && !names.contains(&get_creator_name(set))
                    && get_release_day(set) > day
            });

            if possible_sets.is_empty() {
                // ignore labels
                possible_sets = (0..DIGIT_SETS).collect::<Vec<_>>();
                possible_sets.retain(|&set| {
                    !prev.contains(&set)
                        && next.map_or(true, |next| !next.contains(&set))
                        && !sets.contains(&set)
                        && !names.contains(&get_creator_name(set))
                        && get_release_day(set) >= day
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

            let random = rng.gen::<f64>() * total_weight;

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

        switches[i + 1] = Some(sets);
    }

    let switches = switches.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();

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
