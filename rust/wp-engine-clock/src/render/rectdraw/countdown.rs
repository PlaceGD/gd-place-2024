use core::time;
use std::{array, io::Cursor, sync::LazyLock, time::Instant};

use binrw::BinRead;
use chrono::{DateTime, Local, Timelike};
use glam::{vec2, vec4, Affine2, Vec2};
use rand::{
    rngs::{StdRng, ThreadRng},
    seq::SliceRandom,
    Rng, SeedableRng,
};
use rust_shared::{
    console_log,
    countdown::{CountdownDigitSets, DigitObjects, COLON_COUNT, DIGIT_SETS, TEST_SETS},
    gd::object::{GDColor, GDObject},
    lerp,
    util::random,
};

use crate::{
    config::Config,
    level::{ChunkCoord, Level},
    state::State,
    utilgen::{OBJECT_INFO, SET_SWITCHES},
};

use super::{billy::Billy, draw_level};

pub static COUNTDOWN_DIGITS: LazyLock<CountdownDigitSets> = LazyLock::new(|| {
    let bytes = include_bytes!("../../countdown_digits");

    CountdownDigitSets::read(&mut Cursor::new(bytes)).unwrap()
});

pub struct Countdown {
    pub digits: [CountdownDigit; 6],
    pub state: [Option<u8>; 8],
    pub sets: [usize; 3],

    pub colon_indicies: Vec<usize>,
    pub digit_indicies: Vec<usize>,

    pub hours_marker: Vec<TransitioningObject>,
    pub minutes_marker: Vec<TransitioningObject>,
    pub hours_colon: Vec<TransitioningObject>,
    pub minutes_colon: Vec<TransitioningObject>,
    pub bg_state: [bool; 2],
    pub colon_state: [usize; 2],

    pub first_draw: bool,

    pub prev_switch_id: usize,

    pub rng: StdRng,
}

impl Countdown {
    const OFFSET: Vec2 = vec2(-0.0, 0.0);

    pub fn new(config: &Config) -> Self {
        Self {
            digits: array::from_fn(|_| CountdownDigit::new()),
            state: [None; 8],
            sets: [28, 12, 43],

            colon_indicies: (0..COLON_COUNT).collect(),
            digit_indicies: (0..DIGIT_SETS).collect(),

            hours_marker: COUNTDOWN_DIGITS
                .2
                .objs
                .iter()
                .map(|obj| {
                    TransitioningObject::new(AnimType::Static(*obj), 0.8, true, 0.0, Local::now())
                        .offset(0.2)
                })
                .collect::<Vec<_>>(),
            minutes_marker: COUNTDOWN_DIGITS
                .3
                .objs
                .iter()
                .map(|obj| {
                    TransitioningObject::new(AnimType::Static(*obj), 0.8, true, 0.0, Local::now())
                        .offset(0.2)
                })
                .collect::<Vec<_>>(),

            hours_colon: Vec::new(),
            minutes_colon: Vec::new(),
            bg_state: [true; 2],
            colon_state: [0; 2],

            prev_switch_id: 0,

            first_draw: true,

            rng: StdRng::seed_from_u64(config.general.rng_seed.unwrap_or_else(|| rand::random())),
        }
    }
    pub fn update_state(&mut self, event_start: f64, datetime: DateTime<Local>, config: &Config) {
        //console_log!("{event_start} {now}");
        // let event_elapsed = now / 1000.0 - event_start / 1000.0;
        // let time_until = -event_elapsed;
        let now = datetime.timestamp_millis() as f64;

        let switch_id = (((now / 1000.0) + 0.0).max(0.0)
            / config.sets.digit_change_frequency as f64)
            .floor() as usize;

        let mut has_new_sets = false;

        let (sets, new_colon_state) = if let Some(digits) = &config.sets.sets {
            if self.first_draw {
                has_new_sets = true;

                (
                    [digits.hours, digits.minutes, digits.seconds],
                    if config.sets.show_colons {
                        [digits.colonh, digits.colonm]
                    } else {
                        [0, 0]
                    },
                )
            } else {
                (self.sets, self.colon_state)
            }
        } else {
            if switch_id != self.prev_switch_id {
                self.prev_switch_id = switch_id;
                has_new_sets = true;

                self.colon_indicies.shuffle(&mut self.rng);
                self.digit_indicies.shuffle(&mut self.rng);

                (
                    [
                        self.digit_indicies[0],
                        self.digit_indicies[1],
                        self.digit_indicies[2],
                    ],
                    if config.sets.show_colons {
                        [self.colon_indicies[0], self.colon_indicies[1]]
                    } else {
                        [0, 0]
                    },
                )
            } else {
                (self.sets, self.colon_state)
            }
        };

        // let sets = TEST_SETS.unwrap_or(SET_SWITCHES[switch_id % SET_SWITCHES.len()]);
        //console_log!("{}", switch_id % SET_SWITCHES.len());

        // let days = (time_until / 86400.0).floor();
        // let hours = ((time_until - (days * 86400.0)) / 3600.0).floor();
        // let minutes = ((time_until - (days * 86400.0) - (hours * 3600.0)) / 60.0).floor();
        // let seconds = (time_until - (days * 86400.0) - (hours * 3600.0) - (minutes * 60.0)).floor();

        let hours = datetime.hour();
        let minutes = datetime.minute();
        let seconds = datetime.second();

        fn digits(num: u8) -> (Option<u8>, Option<u8>) {
            (Some(num / 10), Some(num % 10))
        }

        // let (dayd1, dayd2) = digits(days as u8, show_days);
        let (hourd1, hourd2) = digits(hours as u8);
        let (mind1, mind2) = digits(minutes as u8);
        let (secd1, secd2) = digits(seconds as u8);

        let state = [hourd1, hourd2, mind1, mind2, secd1, secd2];
        // let (state, show_days, show_hours, show_minutes) = if time_until < 0.0 {
        //     ([None; 8], false, false, false)
        // } else {
        // };

        for i in 0..self.digits.len() {
            let delay = index_delay(i);

            if sets != self.sets {
                self.digits[i].transition_between(
                    self.state[i],
                    state[i],
                    self.sets[i / 2],
                    sets[i / 2],
                    delay,
                    datetime,
                );
                self.state[i] = state[i];
            } else if self.state[i] != state[i] {
                // let num_sets = COUNTDOWN_DIGITS.0.len();
                // let new_set = if random() < 0.2 {
                //     (random() * num_sets as f64) as usize
                // } else {
                //     self.sets[i]
                // };
                //let new_set = 0;

                self.digits[i].transition_between(
                    self.state[i],
                    state[i],
                    self.sets[i / 2],
                    self.sets[i / 2],
                    delay,
                    datetime,
                );
                self.state[i] = state[i];
                //self.sets[i / 2] = new_set;
            }
        }

        self.sets = sets;

        // colons, days marker
        {
            let appear = |a: &DigitObjects, delay: f32| {
                a.objs
                    .iter()
                    .map(|o| {
                        TransitioningObject::new(
                            if get_alpha(*o) < 0.3 {
                                AnimType::FadeIn(*o)
                            } else {
                                AnimType::Appear(*o, random_axis_offset())
                            },
                            0.8,
                            false,
                            delay,
                            datetime,
                        )
                        .offset(0.2)
                    })
                    .collect::<Vec<_>>()
            };

            let dissapear = |a: &DigitObjects, delay: f32| {
                a.objs
                    .iter()
                    .map(|o| {
                        TransitioningObject::new(
                            if get_alpha(*o) < 0.3 {
                                AnimType::FadeOut(*o)
                            } else {
                                AnimType::Disappear(*o)
                            },
                            0.8,
                            false,
                            delay,
                            datetime,
                        )
                    })
                    .collect::<Vec<_>>()
            };

            // let new_bg_state = [show_hours, show_minutes];

            // if switch_id == 0 {
            //     [0, 0]
            // } else {
            //     [
            //         ((switch_id.wrapping_mul(1103515245).wrapping_add(12345) >> 16) % 6) as usize,
            //         ((switch_id.wrapping_mul(1664525).wrapping_add(1013904223) >> 16) % 6) as usize,
            //     ]
            // };

            if has_new_sets && config.sets.show_colons {
                for i in 0..2 {
                    let (state, prev_colon, colon) = match i {
                        0 => (
                            &mut self.hours_colon,
                            &COUNTDOWN_DIGITS.4[self.colon_state[0] % 6],
                            &COUNTDOWN_DIGITS.4[new_colon_state[0]],
                        ),
                        1 => (
                            &mut self.minutes_colon,
                            &COUNTDOWN_DIGITS.5[self.colon_state[1] % 6],
                            &COUNTDOWN_DIGITS.5[new_colon_state[1]],
                        ),
                        _ => unreachable!(),
                    };

                    let delay = index_delay(i * 2 + 2);
                    state.clear();

                    if !self.first_draw {
                        state.extend(dissapear(prev_colon, delay));
                    }
                    state.extend(appear(colon, delay));

                    self.colon_state[i] = new_colon_state[i];
                }
            }

            self.first_draw = false;

            // self.hours_marker = appear(&COUNTDOWN_DIGITS.2, 1.0);
            // self.minutes_marker = appear(&COUNTDOWN_DIGITS.3, 1.0);

            // for i in 0..2 {
            //     let (state, bg) = match i {
            //         0 => (&mut self.hours_marker, &COUNTDOWN_DIGITS.2),
            //         1 => (&mut self.minutes_marker, &COUNTDOWN_DIGITS.3),
            //         _ => unreachable!(),
            //     };

            //     let delay = index_delay(i * 2);

            //     *state = if new_bg_state[i] {
            //         appear(&bg, delay)
            //     } else {
            //         dissapear(&bg, delay)
            //     };

            //     dbg!(self.bg_state, new_bg_state);

            //     // self.bg_state[i] = new_bg_state[i];
            //     // if self.bg_state[i] != new_bg_state[i] {
            //     // }
            // }
        }
    }

    pub fn draw(&self, state: &State, billy: &mut Billy) {
        let mut level = Level::default();
        let mut idx = 0usize;

        let now = state.now;
        let mut add_object = |mut obj: GDObject| {
            let info = OBJECT_INFO[obj.id as usize];
            obj.ix /= info.builtin_scale_x;
            obj.iy /= info.builtin_scale_x;
            obj.jx /= info.builtin_scale_y;
            obj.jy /= info.builtin_scale_y;
            level.add_object(obj, idx, Some(ChunkCoord { x: 0, y: 0 }), now);
            idx += 1;
        };

        const BLOCK: f32 = 30.0;
        const DIGIT_WIDTH: f32 = 6.0 * BLOCK;
        const COLON_WIDTH: f32 = 4.0 * BLOCK;

        // let position_offset_screen =
        //     (vec2(state.width as f32, state.height as f32) * 0.5) / state.get_zoom_scale();

        // let position_offset_world_vec =
        //     state.get_world_pos(position_offset_screen.x, position_offset_screen.y);

        // let mut position_offset_world =
        //     vec2(position_offset_world_vec[0], position_offset_world_vec[1]);

        // let position = &state.config.clock.position; // a String, one of "center", "top-left", "top-right", "bottom-left", "bottom-right" (defaults to "center")

        // let leftmost_digit = (DIGIT_WIDTH * 2.5) + (BLOCK / 2.0) + BLOCK + COLON_WIDTH;

        // match &position[..] {
        //     "top-left" => {
        //         // let half_screen_width = (state.width as f32) / 2.0;
        //         // // let to_left = (half_screen_width - leftmost_digit).floor();

        //         // position_offset = vec2(0.0, 0.0);
        //         position_offset_world -= vec2(leftmost_digit, 0.0);
        //     }
        //     "top-right" => {
        //         position_offset_world = vec2(0.0, 0.0);
        //     }
        //     "bottom-left" => {
        //         position_offset_world = vec2(0.0, 0.0);
        //     }
        //     "bottom-right" => {
        //         position_offset_world = vec2(0.0, 0.0);
        //     }
        //     // center
        //     _ => {
        //         position_offset_world = vec2(0.0, 0.0);
        //     }
        // }

        let position_offset_world = vec2(0.0, 0.0);

        // state.set_camera_pos(-position_offset.x, 0.0);

        self.hours_marker
            .iter()
            .chain(self.minutes_marker.iter())
            .chain(self.hours_colon.iter())
            .chain(self.minutes_colon.iter())
            .for_each(|o| {
                o.get(state.now).inspect(|o| {
                    add_object(o.offset(vec2(0.0, 0.0)));
                    // level.add_object(*o, idx);
                    // idx += 1;
                });
            });

        // For digit index 0..5
        for (i, digit) in self.digits.iter().enumerate() {
            let mut offset = vec2(0.0, 0.0);

            match i {
                0 => {
                    offset -= vec2(
                        (DIGIT_WIDTH * 2.5) + (BLOCK / 2.0) + BLOCK + COLON_WIDTH,
                        0.0,
                    ) + position_offset_world;
                }
                1 => {
                    offset -= vec2((DIGIT_WIDTH * 1.5) + (BLOCK / 2.0) + COLON_WIDTH, 0.0)
                        + position_offset_world;
                }
                2 => {
                    offset -=
                        vec2((DIGIT_WIDTH * 0.5) + (BLOCK / 2.0), 0.0) + position_offset_world;
                }
                3 => {
                    offset +=
                        vec2((DIGIT_WIDTH * 0.5) + (BLOCK / 2.0), 0.0) - position_offset_world;
                }
                4 => {
                    offset += vec2((DIGIT_WIDTH * 1.5) + (BLOCK / 2.0) + COLON_WIDTH, 0.0)
                        - position_offset_world;
                }
                5 => {
                    offset += vec2(
                        (DIGIT_WIDTH * 2.5) + (BLOCK / 2.0) + BLOCK + COLON_WIDTH,
                        0.0,
                    ) - position_offset_world;
                }
                _ => {}
            }

            for (i, obj) in digit.objects.iter().enumerate() {
                obj.get(state.now).inspect(|o| {
                    // level.add_object(o.offset(offset), idx);
                    // idx += 1;

                    add_object(o.offset(offset));
                });
            }

            // billy.translate(vec2(30.0 * 7.0, 0.0));
        }
        // const BLOCK: f32 = 30.0;
        // const DIGIT_WIDTH: f32 = 6.0 * BLOCK; // 180
        // const COLON_WIDTH: f32 = 3.0 * BLOCK; // 120

        // // Total width of one digit plus intra-pair gap
        // const DIGIT_PLUS_GAP: f32 = DIGIT_WIDTH + BLOCK;

        // // Width of a pair: 2 digits + 1 block between
        // const PAIR_WIDTH: f32 = DIGIT_WIDTH * 2.0 + BLOCK;

        // // Total pair spacing offset (includes 1 colon and 1 extra block)
        // const EXTRA_PAIR_SPACING: f32 = COLON_WIDTH + BLOCK;

        // // For digit index 0..5
        // for (i, digit) in self.digits.iter().enumerate() {
        //     let mut offset = Self::OFFSET;

        //     let pair_index = i / 2;
        //     let in_pair_offset = if i % 2 == 0 {
        //         -DIGIT_PLUS_GAP / 2.0 // left digit in pair
        //     } else {
        //         DIGIT_PLUS_GAP / 2.0 // right digit in pair
        //     };

        //     // Compute total horizontal pair shift from center pair (pair 1 is [2,3])
        //     let pair_offset_from_center = match pair_index {
        //         0 => -PAIR_WIDTH - EXTRA_PAIR_SPACING,
        //         1 => 0.0,
        //         2 => PAIR_WIDTH + EXTRA_PAIR_SPACING,
        //         _ => 0.0,
        //     };

        //     let total_x = pair_offset_from_center + in_pair_offset;
        //     offset += vec2(total_x, 0.0);

        //     for (i, obj) in digit.objects.iter().enumerate() {
        //         obj.get(state.now).inspect(|o| {
        //             // level.add_object(o.offset(offset), idx);
        //             // idx += 1;

        //             add_object(o.offset(offset));
        //         });
        //     }

        //     // billy.translate(vec2(30.0 * 7.0, 0.0));
        // }

        draw_level(state, billy, &level, |_, _, _| None, 0.0, true);

        // self.days_marker.iter().for_each(|o| {
        //     o.get().inspect(|o| draw_obj(o, billy));
        // });

        // self.hours_marker.iter().for_each(|o| {
        //     o.get().inspect(|o| draw_obj(o, billy));
        // });

        // self.minutes_marker.iter().for_each(|o| {
        //     o.get().inspect(|o| draw_obj(o, billy));
        // });

        // billy.translate(Self::OFFSET);

        // for (i, digit) in self.digits.iter().enumerate() {
        //     if i == 2 {
        //         billy.translate(vec2(-30.0 * 14.0, -30.0 * 14.0)); // line break
        //     } else if i != 0 && i % 2 == 0 {
        //         billy.translate(vec2(30.0 * 3.0, 0.0)); // colons
        //     }
        //     digit.draw(billy);
        //     billy.translate(vec2(30.0 * 7.0, 0.0));
        // }
    }
}

pub struct StatsDisplay {
    pub digits: [CountdownDigit; STATS_NUM_DIGITS],
    pub state: [Option<u8>; STATS_NUM_DIGITS],
    pub sets: [usize; STATS_NUM_DIGITS],
}

const STATS_NUM_DIGITS: usize = 7;

impl StatsDisplay {
    pub fn new() -> Self {
        let mut sets = [27287; STATS_NUM_DIGITS];
        for i in 0..STATS_NUM_DIGITS {
            let mut random_set = (random() * DIGIT_SETS as f64) as usize;
            while sets.contains(&random_set) {
                random_set = (random() * DIGIT_SETS as f64) as usize;
            }
            sets[i] = random_set;
        }
        Self {
            digits: array::from_fn(|_| CountdownDigit::new()),
            state: [None; STATS_NUM_DIGITS],
            sets,
        }
    }

    pub fn set_to(&mut self, num: Option<u32>, now: DateTime<Local>) {
        let num_digits: [Option<u8>; STATS_NUM_DIGITS] = if let Some(num) = num {
            array::from_fn(|i| {
                let div = 10u32.pow(STATS_NUM_DIGITS as u32 - i as u32 - 1);
                Some(((num / div) % 10) as u8)
            })
        } else {
            [None; STATS_NUM_DIGITS]
        };

        for i in 0..STATS_NUM_DIGITS {
            let delay = index_delay(i + (8 - STATS_NUM_DIGITS));
            self.digits[i].transition_between(
                self.state[i],
                num_digits[i],
                self.sets[i],
                self.sets[i],
                delay,
                now,
            );
        }
        self.state = num_digits;
    }
    pub fn draw(&self, state: &State, billy: &mut Billy) {
        let mut level = Level::default();
        let mut idx = 0usize;

        let scale = (state.width as f32 / 1920.0).min(state.height as f32 / 1080.0);

        let yoffset = (1920i32 - state.width as i32).max(0) as f32 / 1920.0;

        billy.scale(vec2(scale, scale));

        let mut add_object = |mut obj: GDObject| {
            let info = OBJECT_INFO[obj.id as usize];
            obj.ix /= info.builtin_scale_x;
            obj.iy /= info.builtin_scale_x;
            obj.jx /= info.builtin_scale_y;
            obj.jy /= info.builtin_scale_y;
            level.add_object(obj, idx, Some(ChunkCoord { x: 0, y: 0 }), state.now);
            idx += 1;
        };
        let spacing = 30.0 * 7.5;
        let mut offset = vec2(
            -spacing * (STATS_NUM_DIGITS / 2) as f32,
            -300.0 - yoffset * 300.0,
        );

        for digit in self.digits.iter() {
            for obj in &digit.objects {
                obj.get(state.now).inspect(|o| {
                    add_object(o.offset(offset));
                });
            }

            offset += vec2(spacing, 0.0);
        }

        draw_level(state, billy, &level, |_, _, _| None, 0.0, true);
    }
}

fn index_delay(i: usize) -> f32 {
    (6 - i) as f32 * 0.07
}

fn get_alpha(o: GDObject) -> f32 {
    let mut opacity = (o.main_color.opacity as f32).min(o.detail_color.opacity as f32) / 255.0;
    if o.main_color.blending {
        opacity *= (o.main_color.r as f32 + o.main_color.g as f32 + o.main_color.b as f32)
            .min(o.detail_color.r as f32 + o.detail_color.g as f32 + o.detail_color.b as f32)
            / 3.0
            / 255.0;
    }
    opacity
}

struct CountdownDigit {
    objects: Vec<TransitioningObject>,
}

impl CountdownDigit {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    fn from_set(set: usize, digit: u8, now: DateTime<Local>) -> Self {
        let mut empty = Self::new();
        empty.set_to(set, digit, 0.5, now);
        empty
    }

    // fn draw(&self, billy: &mut Billy) {
    //     for obj in &self.objects {
    //         obj.get(state.now).inspect(|o| draw_obj(o, billy));
    //     }
    // }

    fn get_set(set: usize, digit: u8) -> &'static [GDObject] {
        &COUNTDOWN_DIGITS.0[set].0[(digit as usize) % 10].objs
    }

    fn set_to(&mut self, set: usize, digit: u8, duration: f64, now: DateTime<Local>) {
        self.objects = Self::get_set(set, digit)
            .iter()
            .map(|o| {
                TransitioningObject::new(
                    AnimType::Appear(*o, vec2(0.0, 0.0)),
                    duration,
                    true,
                    0.0,
                    now,
                )
            })
            .collect();
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn transition_between(
        &mut self,
        digit1: Option<u8>,
        digit2: Option<u8>,
        set1: usize,
        set2: usize,
        delay: f32,
        now: DateTime<Local>,
    ) {
        self.objects.clear(); // so it dont reallocate :)

        let setup = |digit: Option<u8>, set: usize| {
            digit
                .map(|d| Self::get_set(set, d))
                .unwrap_or(&[])
                .into_iter()
                //.filter(|o| matches!(o.z_layer, rust_shared::gd::layer::ZLayer::B1))
                .copied()
                .map(|o| (o, true))
                .collect::<Vec<_>>()
        };

        let mut trans_out = setup(digit1, set1);
        let mut trans_in = setup(digit2, set2);
        let mut static_objs = Vec::new();

        // unchanging objects
        for obj_out in trans_out.iter_mut().filter(|o| o.1) {
            for obj_in in trans_in.iter_mut().filter(|o| o.1) {
                if obj_in.0.id == obj_out.0.id
                    && same_pos(obj_out.0, obj_in.0)
                    && same_colors(obj_out.0, obj_in.0)
                    && same_transform(obj_out.0, obj_in.0)
                {
                    obj_out.1 = false;
                    obj_in.1 = false;
                    static_objs.push(obj_out.0);
                    self.objects.push(TransitioningObject::new(
                        AnimType::Static(obj_out.0),
                        0.8,
                        false,
                        delay,
                        now,
                    ));
                    break;
                }
            }
        }

        trans_out.retain(|o| o.1);
        trans_in.retain(|o| o.1);

        // // changing but just barely
        // for obj_out in &mut trans_out {
        //     if !obj_out.1 {
        //         continue;
        //     }
        //     for obj_in in &mut trans_in {
        //         if !obj_in.1 {
        //             continue;
        //         }
        //         if obj_in.0.id == obj_out.0.id
        //             && same_pos(obj_out.0, obj_in.0)
        //             && same_colors(obj_out.0, obj_in.0)
        //             && symmetrical_transform(obj_out.0, obj_in.0)
        //         {
        //             obj_out.1 = false;
        //             obj_in.1 = false;
        //             static_objs.push(obj_in.0);
        //             self.objects.push(TransitioningObject::new(
        //                 AnimType::TinyTransition(obj_out.0, obj_in.0),
        //                 0.8,
        //                 false,
        //             ));
        //         }
        //     }
        // }

        // trans_out.retain(|o| o.1);
        // trans_in.retain(|o| o.1);

        // copied from static objects

        for obj_in in trans_in.iter_mut().filter(|o| o.1) {
            for obj_static in &static_objs {
                if obj_in.0.id == obj_static.id
                    && obj_chebyshev_dist(obj_in.0, *obj_static) < 45.0
                    && close_colors(obj_in.0, *obj_static)
                    && obj_angle_dist(obj_in.0, *obj_static) <= std::f32::consts::PI / 2.0 + 0.1
                    && obj_scale_difference(obj_in.0, *obj_static) <= 3.0
                {
                    obj_in.1 = false;
                    self.objects.push(TransitioningObject::new(
                        AnimType::Copy(*obj_static, obj_in.0, random() < 0.5),
                        0.8,
                        false,
                        delay,
                        now,
                    ));
                    break;
                }
            }
        }

        trans_in.retain(|o| o.1);

        // transitioning from a to b
        for obj_out in trans_out.iter_mut().filter(|o| o.1) {
            for obj_in in trans_in.iter_mut().filter(|o| o.1) {
                // if obj_in.0.id == 3810 {
                //     console_log!("{} {}", obj_chebyshev_dist(obj_out.0, obj_in.0), obj_in.0.y);
                // }
                if obj_in.0.id == obj_out.0.id
                    && obj_chebyshev_dist(obj_out.0, obj_in.0) <= 60.0
                    && close_colors(obj_out.0, obj_in.0)
                    && obj_scale_difference(obj_out.0, obj_in.0) <= 3.0
                //&& same_transform(obj_out.0, obj_in.0)
                {
                    obj_in.1 = false;
                    obj_out.1 = false;
                    self.objects.push(TransitioningObject::new(
                        AnimType::Transition(obj_out.0, obj_in.0, random() < 0.5),
                        0.7,
                        false,
                        delay,
                        now,
                    ));
                    break;
                }
            }
        }

        // longer transition
        for obj_out in trans_out.iter_mut().filter(|o| o.1) {
            for obj_in in trans_in.iter_mut().filter(|o| o.1) {
                // if obj_in.0.id == 3810 {
                //     console_log!("{} {}", obj_chebyshev_dist(obj_out.0, obj_in.0), obj_in.0.y);
                // }
                if obj_in.0.id == obj_out.0.id
                    && obj_chebyshev_dist(obj_out.0, obj_in.0) < 90.0
                    && close_colors(obj_out.0, obj_in.0)
                    && same_transform(obj_out.0, obj_in.0)
                {
                    obj_in.1 = false;
                    obj_out.1 = false;
                    self.objects.push(TransitioningObject::new(
                        AnimType::Transition(obj_out.0, obj_in.0, random() < 0.5),
                        0.7,
                        false,
                        delay,
                        now,
                    ));
                    break;
                }
            }
        }

        self.objects.extend(trans_out.iter().filter_map(|o| {
            o.1.then(|| TransitioningObject::new(AnimType::Disappear(o.0), 0.8, true, delay, now))
        }));

        self.objects.extend(trans_in.iter().filter_map(|o| {
            o.1.then(|| {
                TransitioningObject::new(
                    AnimType::Appear(o.0, random_axis_offset()),
                    0.8,
                    true,
                    delay,
                    now,
                )
            })
        }));

        // resort (dw this only happens once a second to like 200 objects at a time)
        // console_log!("FUCK FART");
        // self.objects.sort_by(|a, b| {
        //     let a = a.typ.output_obj();
        //     let b = b.typ.output_obj();
        //     // place blending objects at bottom of layer (dont care about detail color for this)
        //     let a_z = if a.main_color.blending {
        //         i8::MIN
        //     } else {
        //         a.z_order
        //     };
        //     let b_z = if b.main_color.blending {
        //         i8::MIN
        //     } else {
        //         b.z_order
        //     };
        //     (a.z_layer as u8)
        //         .cmp(&(b.z_layer as u8))
        //         .then(a_z.cmp(&b_z))
        // });
    }
}

fn random_axis_offset() -> Vec2 {
    if random() < 0.5 {
        vec2(random() as f32 - 0.5, 0.0) * 30.0
    } else {
        vec2(0.0, random() as f32 - 0.5) * 30.0
    }
}

// pub fn is_equivalent(obj1: GDObject, obj2: GDObject, allow_more: bool) -> bool {
//     if obj1.id != obj2.id {
//         return false;
//     }
//     let same_pos = obj_chebyshev_dist(obj1, obj2) < 0.1;
//     let same_transform = if allow_more {
//         symmetrical_transform(obj1, obj2)
//     } else {
//         same_transform(obj1, obj2)
//     };

//     let same_colors = if allow_more {
//         obj1.main_color.blending == obj2.main_color.blending
//             && obj1.detail_color.blending == obj2.detail_color.blending
//     } else {
//         same_colors(obj1, obj2)
//     };

//     same_pos && same_transform && same_colors
// }

fn same_pos(obj1: GDObject, obj2: GDObject) -> bool {
    obj_chebyshev_dist(obj1, obj2) < 0.1
}

fn same_colors(obj1: GDObject, obj2: GDObject) -> bool {
    obj1.main_color == obj2.main_color && obj1.detail_color == obj2.detail_color
}

fn close_colors(obj1: GDObject, obj2: GDObject) -> bool {
    color_close(obj1.main_color, obj2.main_color)
        && color_close(obj1.detail_color, obj2.detail_color)
}

fn same_transform(obj1: GDObject, obj2: GDObject) -> bool {
    (obj1.ix - obj2.ix).abs() < 0.02
        && (obj1.iy - obj2.iy).abs() < 0.02
        && (obj1.jx - obj2.jx).abs() < 0.02
        && (obj1.jy - obj2.jy).abs() < 0.02
}

fn symmetrical_transform(obj1: GDObject, obj2: GDObject) -> bool {
    // symmetry
    ((obj1.ix + obj2.ix).abs() < 0.02 || (obj1.ix - obj2.ix).abs() < 0.02)
        && ((obj1.iy + obj2.iy).abs() < 0.02 || (obj1.iy - obj2.iy).abs() < 0.02)
        && ((obj1.jx + obj2.jx).abs() < 0.02 || (obj1.jx - obj2.jx).abs() < 0.02)
        && ((obj1.jy + obj2.jy).abs() < 0.02 || (obj1.jy - obj2.jy).abs() < 0.02)
        // rotation 90 deg
    || ((obj1.ix + obj2.iy).abs() < 0.02 || (obj1.ix - obj2.iy).abs() < 0.02)
        && ((obj1.iy + obj2.ix).abs() < 0.02 || (obj1.iy - obj2.ix).abs() < 0.02)
        && ((obj1.jx + obj2.jy).abs() < 0.02 || (obj1.jx - obj2.jy).abs() < 0.02)
        && ((obj1.jy + obj2.jx).abs() < 0.02 || (obj1.jy - obj2.jx).abs() < 0.02)
}

fn obj_scale_difference(obj1: GDObject, obj2: GDObject) -> f32 {
    let scalex1 = vec2(obj1.ix, obj1.iy).length();
    let scalex2 = vec2(obj2.ix, obj2.iy).length();
    let scaley1 = vec2(obj1.jx, obj1.jy).length();
    let scaley2 = vec2(obj2.jx, obj2.jy).length();

    (scalex1 / scalex2)
        .max(scalex2 / scalex1)
        .max(scaley1 / scaley2)
        .max(scaley2 / scaley1)
}

fn obj_chebyshev_dist(obj1: GDObject, obj2: GDObject) -> f32 {
    (obj1.x - obj2.x).abs().max((obj1.y - obj2.y).abs())
}

fn obj_angle_dist(obj1: GDObject, obj2: GDObject) -> f32 {
    let angle1 = vec2(obj1.ix, obj1.iy).angle_to(vec2(obj2.ix, obj2.iy));
    let angle2 = vec2(obj1.jx, obj1.jy).angle_to(vec2(obj2.jx, obj2.jy));
    let angle1 = if angle1 > std::f32::consts::PI {
        angle1 - std::f32::consts::PI * 2.0
    } else {
        angle1
    };
    let angle2 = if angle2 > std::f32::consts::PI {
        angle2 - std::f32::consts::PI * 2.0
    } else {
        angle2
    };
    angle1.abs().max(angle2.abs())
}

fn ease_out_quart(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(4)
}

fn color_close(col1: GDColor, col2: GDColor) -> bool {
    if col1.blending != col2.blending {
        return false;
    }

    let d = |n| n as f32 / 255.0;

    (d(col1.r) - d(col2.r)).powi(2)
        + (d(col1.g) - d(col2.g)).powi(2)
        + (d(col1.b) - d(col2.b)).powi(2)
        + (d(col1.opacity) - d(col2.opacity)).powi(2)
        < 0.2 * 0.2
}

enum AnimType {
    Appear(GDObject, Vec2), // Vec2: offset
    FadeIn(GDObject),
    Disappear(GDObject),
    FadeOut(GDObject),
    Transition(GDObject, GDObject, bool), // bool: prefer x axis

    Copy(GDObject, GDObject, bool), // bool: prefer x axis

    TinyTransition(GDObject, GDObject),
    Static(GDObject),
}

impl AnimType {
    fn output_obj(&self) -> GDObject {
        *match self {
            AnimType::Appear(obj, _) => obj,
            AnimType::Disappear(obj) => obj,
            AnimType::Transition(_, obj, _) => obj,
            AnimType::Static(obj) => obj,
            AnimType::TinyTransition(_, obj) => obj,
            AnimType::Copy(_, obj, _) => obj,
            AnimType::FadeIn(obj) => obj,
            AnimType::FadeOut(obj) => obj,
        }
    }
}

struct TransitioningObject {
    typ: AnimType,
    start_time: f64,
    duration: f64,
}

impl TransitioningObject {
    fn get(&self, now: DateTime<Local>) -> Option<GDObject> {
        let time = now.timestamp_millis() as f64 / 1000.0;
        let d = (time - self.start_time) / self.duration;

        // dbg!(now.timestamp_millis());

        fn lerp_color(c1: GDColor, c2: GDColor, d: f64) -> GDColor {
            GDColor {
                r: lerp!(c1.r as f64, c2.r as f64, d) as u8,
                g: lerp!(c1.g as f64, c2.g as f64, d) as u8,
                b: lerp!(c1.b as f64, c2.b as f64, d) as u8,
                opacity: lerp!(c1.opacity as f64, c2.opacity as f64, d) as u8,
                blending: c1.blending,
            }
        }

        match self.typ {
            AnimType::Static(obj) => Some(obj),
            AnimType::TinyTransition(obj1, obj2) => {
                let main_color = lerp_color(obj1.main_color, obj2.main_color, d);
                let detail_color = lerp_color(obj1.detail_color, obj2.detail_color, d);

                let mut obj = if d < 0.5 { obj1 } else { obj2 };
                obj.main_color = main_color;
                obj.detail_color = detail_color;
                Some(obj)
            }
            AnimType::Appear(obj, offset) => {
                if d < 0.4 {
                    None
                } else if d < 0.8 {
                    let mut edited_obj = obj;
                    {
                        let d = ((d - 0.4) / 0.4).max(0.0).min(1.0);
                        let div = 3.0;
                        let i = 1.0 - (d as f32 * div).floor() / div;

                        edited_obj.x = obj.x + offset.x * i;
                        edited_obj.y = obj.y + offset.y * i;
                    }

                    {
                        edited_obj.ix = 1.0;
                        edited_obj.iy = 0.0;
                        edited_obj.jx = 0.0;
                        edited_obj.jy = 1.0;
                        let d = ((d - 0.45) / 0.3).max(0.0).min(1.0);
                        if let Some(value) = transform_animation(&mut edited_obj, obj, d) {
                            return Some(value);
                        }
                    }
                    // Some(if obj.id == 1888 {
                    //     // glow not tinted because ugly
                    //     edited_obj
                    // } else {
                    //     edited_obj.select_tint()
                    // })
                    Some(edited_obj.select_tint())
                } else {
                    Some(obj)
                }
            }
            AnimType::Disappear(obj) => {
                if d < 0.3 {
                    Some(obj)
                } else if d < 0.45 {
                    Some(obj.select_tint())
                } else {
                    None
                }
            }
            AnimType::Copy(mut obj, target, prefer_x_axis) => {
                if let Some(value) = move_obj_animation(&mut obj, target, d, prefer_x_axis) {
                    return Some(value);
                }

                Some(obj.copypaste_tint())
            }
            AnimType::Transition(mut obj, target, prefer_x_axis) => {
                if let Some(value) = move_obj_animation(&mut obj, target, d, prefer_x_axis) {
                    return Some(value);
                }

                Some(obj.select_tint())
            }
            AnimType::FadeIn(mut obj) => {
                let d = d.max(0.0).min(1.0);
                obj.main_color.opacity = (obj.main_color.opacity as f64 * d as f64) as u8;
                obj.detail_color.opacity = (obj.detail_color.opacity as f64 * d as f64) as u8;
                Some(obj)
            }
            AnimType::FadeOut(mut obj) => {
                let d = d.max(0.0).min(1.0);
                obj.main_color.opacity = (obj.main_color.opacity as f64 * (1.0 - d) as f64) as u8;
                obj.detail_color.opacity =
                    (obj.detail_color.opacity as f64 * (1.0 - d) as f64) as u8;
                Some(obj)
            }
        }
    }

    fn new(
        typ: AnimType,
        duration: f64,
        y_delay: bool,
        mut delay: f32,
        now: DateTime<Local>,
    ) -> TransitioningObject {
        let time = now.timestamp_millis() as f64 / 1000.0;
        if y_delay {
            delay += typ.output_obj().y / 300.0 * 0.25
        }
        TransitioningObject {
            typ,
            start_time: time + random() * duration * 0.1 + delay as f64,
            duration: duration * 0.9,
        }
    }

    fn offset(mut self, s: f64) -> Self {
        self.start_time += s;
        self
    }
}

fn move_obj_animation(
    obj: &mut GDObject,
    target: GDObject,
    d: f64,
    prefer_x_axis: bool,
) -> Option<GDObject> {
    if d >= 0.999 {
        return Some(target);
    } else if d < 0.001 {
        return Some(*obj);
    }
    let move_x = target.x - obj.x;
    let move_y = target.y - obj.y;

    if d > 0.5 {
        obj.z_layer = target.z_layer;
    }

    obj.z_order = lerp!(obj.z_order as f64, target.z_order as f64, d) as i8;

    if move_x != 0.0 || move_y != 0.0 {
        let first_axis =
            if (prefer_x_axis && move_x.abs() == move_y.abs()) || move_x.abs() > move_y.abs() {
                let (a, b) = (obj.x, obj.y);
                (
                    &mut obj.x,
                    &mut obj.y,
                    a / 30.0,
                    b / 30.0,
                    move_x / 30.0,
                    move_y / 30.0,
                )
            } else {
                let (a, b) = (obj.y, obj.x);
                (
                    &mut obj.y,
                    &mut obj.x,
                    a / 30.0,
                    b / 30.0,
                    move_y / 30.0,
                    move_x / 30.0,
                )
            };

        let first_axis_time = 0.6
            * (first_axis.4.abs() as f64 / (first_axis.4.abs() as f64 + first_axis.5.abs() as f64));
        let second_axis_time = 0.6
            * (first_axis.5.abs() as f64 / (first_axis.5.abs() as f64 + first_axis.4.abs() as f64));

        if d < first_axis_time {
            // fractional move first axis
            let div = first_axis.4.floor().max(1.0) * 4.0;

            let d = (d) / first_axis_time;
            let i = (d as f32 * div).floor() / div;
            *first_axis.0 = (first_axis.2 + first_axis.4 * i) * 30.0;
        } else if d < first_axis_time + second_axis_time {
            // fractional move second axis
            let div = first_axis.5.floor().max(1.0) * 4.0;

            let d = (d - first_axis_time) / second_axis_time;
            let i = (d as f32 * div).floor() / div;
            *first_axis.1 = (first_axis.3 + first_axis.5 * i) * 30.0;

            *first_axis.0 = (first_axis.2 + first_axis.4) * 30.0;
        } else {
            *first_axis.0 = (first_axis.2 + first_axis.4) * 30.0;
            *first_axis.1 = (first_axis.3 + first_axis.5) * 30.0;
        }
    }

    if d > 0.2 && within_90_deg(*obj, target) {
        let mut i1 = vec2(obj.ix, obj.iy);
        let i2 = vec2(target.ix, target.iy);
        let mut j1 = vec2(obj.jx, obj.jy);
        let j2 = vec2(target.jx, target.jy);
        i1 = (vec2(target.ix, target.iy) / i2.length()) * i1.length();
        j1 = (vec2(target.jx, target.jy) / j2.length()) * j1.length();

        obj.ix = i1.x;
        obj.iy = i1.y;
        obj.jx = j1.x;
        obj.jy = j1.y;
    }

    if d >= 0.5 {
        if same_transform(*obj, target) {
            return Some(target);
        }
        if let Some(value) = transform_animation(obj, target, (d - 0.5) / 0.5) {
            return Some(value);
        }
    }
    None
}

fn transform_animation(obj: &mut GDObject, target: GDObject, d: f64) -> Option<GDObject> {
    let mut i1 = vec2(obj.ix, obj.iy);
    let i2 = vec2(target.ix, target.iy);
    let mut j1 = vec2(obj.jx, obj.jy);
    let j2 = vec2(target.jx, target.jy);
    let i1len = i1.length();
    let i2len = i2.length();
    let j1len = j1.length();
    let j2len = j2.length();
    if within_90_deg(*obj, target) {
        if d > 0.8 || ((i1len - i2len).abs() < 0.01 && (j1len - j2len).abs() < 0.01) {
            return Some(target);
        }
        i1 = i2 / i2len;
        j1 = j2 / j2len;
        let scale = ((d as f32) / 0.6).max(0.0).min(1.0);

        i1 *= lerp!(i1len, i2len, scale);
        j1 *= lerp!(j1len, j2len, scale);
    } else {
        let rot = ((d as f32) / 0.4).max(0.0).min(1.0);

        i1 = rotate_between(i1 / i1len, i2 / i2len, rot);
        j1 = rotate_between(j1 / j1len, j2 / j2len, rot);

        let scale = ease_out_quart(((d as f32 - 0.4) / 0.5).max(0.0).min(1.0));

        i1 *= lerp!(i1len, i2len, scale);
        j1 *= lerp!(j1len, j2len, scale);
    }
    obj.ix = i1.x;
    obj.iy = i1.y;
    obj.jx = j1.x;
    obj.jy = j1.y;
    // rotate

    None
}

fn within_90_deg(obj: GDObject, target: GDObject) -> bool {
    (obj_angle_dist(obj, target).abs() - std::f32::consts::PI / 2.0).abs() < 0.1
        || (obj_angle_dist(obj, target).abs() - std::f32::consts::PI).abs() < 0.1
}

fn rotate_between(a: Vec2, b: Vec2, d: f32) -> Vec2 {
    let angle = a.angle_to(b);
    let angle = if angle > std::f32::consts::PI {
        angle - std::f32::consts::PI * 2.0
    } else {
        angle
    };
    let angle = angle * d;
    let rot = Affine2::from_angle(angle);
    rot.transform_vector2(a)
}

fn closest_iterator(range: std::ops::Range<usize>, x: usize) -> impl Iterator<Item = usize> {
    let mut sequence = Vec::with_capacity((range.end - range.start) as usize);
    let start = range.start;
    let end = range.end;

    let max_distance = std::cmp::max(x - start, end - 1 - x);

    for offset in 0..=max_distance {
        if offset == 0 {
            if x >= start && x < end {
                sequence.push(x);
            }
        } else {
            let right = x + offset;
            if right < end {
                sequence.push(right);
            }
            let left = x - offset;
            if left >= start {
                sequence.push(left);
            }
        }
    }

    sequence.into_iter()
}
