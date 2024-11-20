use std::{array, io::Cursor, sync::LazyLock, time::Instant};

use binrw::BinRead;
use glam::{vec2, vec4, Affine2, Vec2};
use rust_shared::{
    console_log,
    countdown::{CountdownDigitSets, DigitObjects, DIGIT_SETS, TEST_SETS},
    gd::object::{GDColor, GDObject},
    lerp,
    util::random,
};

use crate::{
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
    pub digits: [CountdownDigit; 8],
    pub state: [Option<u8>; 8],
    pub sets: [usize; 4],

    pub days_marker: Vec<TransitioningObject>,
    pub hours_marker: Vec<TransitioningObject>,
    pub minutes_marker: Vec<TransitioningObject>,
    pub hours_colon: Vec<TransitioningObject>,
    pub minutes_colon: Vec<TransitioningObject>,
    pub bg_state: [bool; 3],
    pub colon_state: [usize; 2],
}

impl Countdown {
    const OFFSET: Vec2 = vec2(450.0, 390.0 + 30.0 * 14.0);

    pub fn new() -> Self {
        Self {
            digits: array::from_fn(|_| CountdownDigit::new()),
            state: [None; 8],
            sets: [28, 3, 12, 43],

            days_marker: Vec::new(),
            hours_marker: Vec::new(),
            minutes_marker: Vec::new(),

            hours_colon: Vec::new(),
            minutes_colon: Vec::new(),
            bg_state: [false; 3],
            colon_state: [1000; 2],
        }
    }
    pub fn update_state(&mut self, event_start: f64, now: f64) {
        //console_log!("{event_start} {now}");
        let event_elapsed = now / 1000.0 - event_start / 1000.0;
        let time_until = -event_elapsed;

        if time_until.is_nan() || time_until.is_infinite() {
            return;
        }

        // if u change this also change it in the wasm :3
        let switch_id = ((time_until + 600.0).max(0.0) / 1200.0).floor() as usize;
        //console_log!("{time_until}");

        let sets = TEST_SETS.unwrap_or(SET_SWITCHES[switch_id % SET_SWITCHES.len()]);
        //console_log!("{}", switch_id % SET_SWITCHES.len());

        let (state, show_days, show_hours, show_minutes) = if time_until < 0.0 {
            ([None; 8], false, false, false)
        } else {
            let days = (time_until / 86400.0).floor();
            let hours = ((time_until - (days * 86400.0)) / 3600.0).floor();
            let minutes = ((time_until - (days * 86400.0) - (hours * 3600.0)) / 60.0).floor();
            let seconds =
                (time_until - (days * 86400.0) - (hours * 3600.0) - (minutes * 60.0)).floor();

            let show_days = days != 0.0;
            let show_hours = true; //show_days || hours != 0.0;
            let show_minutes = true; //show_hours || minutes != 0.0;

            fn digits(num: u8, display: bool) -> (Option<u8>, Option<u8>) {
                if !display {
                    (None, None)
                } else {
                    (Some(num / 10), Some(num % 10))
                }
            }

            let (dayd1, dayd2) = digits(days as u8, show_days);
            let (hourd1, hourd2) = digits(hours as u8, show_hours);
            let (mind1, mind2) = digits(minutes as u8, show_minutes);
            let (secd1, secd2) = digits(seconds as u8, true);

            (
                [dayd1, dayd2, hourd1, hourd2, mind1, mind2, secd1, secd2],
                show_days,
                show_hours,
                show_minutes,
            )
        };

        for i in 0..8 {
            let delay = index_delay(i);
            if sets != self.sets {
                self.digits[i].transition_between(
                    self.state[i],
                    state[i],
                    self.sets[i / 2],
                    sets[i / 2],
                    delay,
                    now,
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
                    now,
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
                            now,
                        )
                        .offset(0.2)
                    })
                    .collect()
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
                            now,
                        )
                    })
                    .collect()
            };

            let new_bg_state = [show_days, show_hours, show_minutes];

            let new_colon_state = if switch_id == 0 {
                [0, 0]
            } else {
                [
                    ((switch_id.wrapping_mul(1103515245).wrapping_add(12345) >> 16) % 6) as usize,
                    ((switch_id.wrapping_mul(1664525).wrapping_add(1013904223) >> 16) % 6) as usize,
                ]
            };

            for i in 0..2 {
                if self.bg_state[i + 1] != new_bg_state[i + 1]
                    || self.colon_state[i] != new_colon_state[i]
                {
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

                    if self.bg_state[i + 1] {
                        state.extend(dissapear(prev_colon, delay));
                    }
                    if new_bg_state[i + 1] {
                        state.extend(appear(colon, delay));
                    }

                    self.colon_state[i] = new_colon_state[i];
                }
            }

            for i in 0..3 {
                if self.bg_state[i] != new_bg_state[i] {
                    let (state, bg) = match i {
                        0 => (&mut self.days_marker, &COUNTDOWN_DIGITS.1),
                        1 => (&mut self.hours_marker, &COUNTDOWN_DIGITS.2),
                        2 => (&mut self.minutes_marker, &COUNTDOWN_DIGITS.3),
                        _ => unreachable!(),
                    };
                    let delay = index_delay(i * 2);
                    *state = if new_bg_state[i] {
                        appear(&bg, delay)
                    } else {
                        dissapear(&bg, delay)
                    };

                    self.bg_state[i] = new_bg_state[i];
                }
            }
        }
    }

    pub fn draw(&self, state: &State, billy: &mut Billy) {
        let mut level = Level::default();
        let mut idx = 0usize;

        let mut add_object = |mut obj: GDObject| {
            let info = OBJECT_INFO[obj.id as usize];
            obj.ix /= info.builtin_scale_x;
            obj.iy /= info.builtin_scale_x;
            obj.jx /= info.builtin_scale_y;
            obj.jy /= info.builtin_scale_y;
            level.add_object(obj, idx, Some(ChunkCoord { x: 0, y: 0 }), state.now);
            idx += 1;
        };

        let mut offset = Self::OFFSET;

        self.days_marker
            .iter()
            .chain(self.hours_marker.iter())
            .chain(self.minutes_marker.iter())
            .chain(self.hours_colon.iter())
            .chain(self.minutes_colon.iter())
            .for_each(|o| {
                o.get(state.now).inspect(|o| {
                    add_object(o.offset(offset - vec2(450.0, 450.0 + 30.0 * 14.0)));
                    // level.add_object(*o, idx);
                    // idx += 1;
                });
            });

        for (i, digit) in self.digits.iter().enumerate() {
            if i == 2 {
                offset += vec2(-30.0 * 14.0, -30.0 * 14.0); // line break
            } else if i != 0 && i % 2 == 0 {
                offset += vec2(30.0 * 3.0, 0.0); // colons
            }
            // digit.draw(billy);

            for obj in &digit.objects {
                obj.get(state.now).inspect(|o| {
                    // level.add_object(o.offset(offset), idx);
                    // idx += 1;
                    add_object(o.offset(offset));
                });
            }

            offset += vec2(30.0 * 7.0, 0.0);

            // billy.translate(vec2(30.0 * 7.0, 0.0));
        }

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

    pub fn set_to(&mut self, num: Option<u32>, now: f64) {
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

pub struct AllDesignsDisplay {
    pub digits: [CountdownDigit; DIGIT_SETS],
    pub state: [u8; DIGIT_SETS],

    pub sets: ([usize; 24], [usize; 24], [usize; 24], [usize; 13]),
}

impl AllDesignsDisplay {
    pub fn new() -> Self {
        let pretty_sets = (0..DIGIT_SETS)
            .into_iter()
            .filter(|i| {
                rust_shared::countdown::get_set_labels(*i).pretty
                    && !rust_shared::countdown::get_set_labels(*i).silly
            })
            .collect::<Vec<_>>();

        let mut pretty1 = pretty_sets[0..24].to_vec();
        let mut pretty2 = pretty_sets[24..38].to_vec();

        console_log!("{:?}", pretty_sets.len());

        let mut silly_sets = (0..DIGIT_SETS)
            .into_iter()
            .filter(|i| {
                rust_shared::countdown::get_set_labels(*i).silly
                    && !rust_shared::countdown::get_set_labels(*i).pretty
            })
            .collect::<Vec<_>>();

        let mut remaining = (0..DIGIT_SETS).into_iter().filter(|i| {
            !(rust_shared::countdown::get_set_labels(*i).pretty
                && !rust_shared::countdown::get_set_labels(*i).silly)
                && !(rust_shared::countdown::get_set_labels(*i).silly
                    && !rust_shared::countdown::get_set_labels(*i).pretty)
        });

        // add to pretty2 and silly_sets until they are length 24
        for i in 0..24 {
            if pretty2.len() < 24 {
                pretty2.push(remaining.next().unwrap());
            }
            if silly_sets.len() < 24 {
                silly_sets.push(remaining.next().unwrap());
            }
        }

        let mut last_group = remaining.collect::<Vec<_>>();

        // sort them such that famous sets are first, then by average weight
        let predicate = |i: &usize| {
            let labels = rust_shared::countdown::get_set_labels(*i);
            let weight = rust_shared::countdown::get_set_weights(*i)
                .iter()
                .sum::<f64>()
                / 4.0;
            let famous = labels.famous;
            (if famous {
                -weight * 100.0 - 100000.0
            } else {
                -weight * 100.0
            }) as i32
        };
        pretty1.sort_by_key(predicate);
        pretty2.sort_by_key(predicate);
        silly_sets.sort_by_key(predicate);
        last_group.sort_by_key(predicate);

        console_log!(
            "{} {} {} {}",
            pretty1.len(),
            pretty2.len(),
            silly_sets.len(),
            last_group.len(),
        );

        // convert to arrays
        let group1: [usize; 24] = pretty1.try_into().unwrap();
        let group2: [usize; 24] = pretty2.try_into().unwrap();
        let group3: [usize; 24] = silly_sets.try_into().unwrap();
        let group4: [usize; 13] = last_group.try_into().unwrap();

        let mut out = Self {
            digits: array::from_fn(|_| CountdownDigit::new()),
            state: [0; DIGIT_SETS],
            sets: (group1, group2, group3, group4),
        };

        for i in 0..DIGIT_SETS {
            out.digits[i] = CountdownDigit::from_set(out.get_set(i), 0, 0.0);
        }

        out
    }

    pub fn get_set(&self, set: usize) -> usize {
        match set {
            0..=23 => self.sets.0[set],
            24..=47 => self.sets.1[set - 24],
            48..=71 => self.sets.2[set - 48],
            72..=84 => self.sets.3[set - 72],
            _ => unreachable!(),
        }
    }

    pub fn get_position(&self, set: usize) -> Vec2 {
        let (group, i) = match set {
            0..=23 => (0, set),
            24..=47 => (1, set - 24),
            48..=71 => (2, set - 48),
            72..=84 => (3, set - 72),
            _ => unreachable!(),
        };

        let offsets = [
            vec2(450.0, 30.0 * 16.0 * 5.0),
            vec2(450.0, 30.0 * 16.0 * 11.0),
            vec2(450.0 + 30.0 * 9.0 * 8.0, 30.0 * 16.0 * 5.0),
            vec2(450.0 + 30.0 * 9.0 * 8.0, 30.0 * 16.0 * 11.0),
        ];

        offsets[group]
            + vec2(
                30.0 * 10.0 * ((i % 6) as f32),
                -30.0 * 16.0 * ((i / 6) as f32),
            )
    }

    pub fn increment(&mut self, index: usize, now: f64) {
        let new_val = (self.state[index] + 1) % 10;
        self.digits[index].transition_between(
            Some(self.state[index]),
            Some(new_val),
            self.get_set(index),
            self.get_set(index),
            0.0,
            now,
        );
        self.state[index] = new_val;
    }

    pub fn draw(&self, state: &State, billy: &mut Billy) {
        let mut level = Level::default();
        let mut idx = 0usize;

        let mut add_object = |mut obj: GDObject| {
            let info = OBJECT_INFO[obj.id as usize];
            obj.ix /= info.builtin_scale_x;
            obj.iy /= info.builtin_scale_x;
            obj.jx /= info.builtin_scale_y;
            obj.jy /= info.builtin_scale_y;
            level.add_object(obj, idx, Some(ChunkCoord { x: 0, y: 0 }), state.now);
            idx += 1;
        };

        for (i, digit) in self.digits.iter().enumerate() {
            let offset = self.get_position(i);

            for obj in &digit.objects {
                obj.get(state.now).inspect(|o| {
                    add_object(o.offset(offset));
                });
            }
        }

        draw_level(state, billy, &level, |_, _, _| None, 0.0, true);
    }
}

fn index_delay(i: usize) -> f32 {
    (8 - i) as f32 * 0.07
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
    fn from_set(set: usize, digit: u8, now: f64) -> Self {
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

    fn set_to(&mut self, set: usize, digit: u8, duration: f64, now: f64) {
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
        now: f64,
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
    fn get(&self, now: f64) -> Option<GDObject> {
        let time = now / 1000.0;
        let d = (time - self.start_time) / self.duration;

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
        now: f64,
    ) -> TransitioningObject {
        let time = now / 1000.0;
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
