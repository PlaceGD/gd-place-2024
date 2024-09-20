use std::{array, io::Cursor, sync::LazyLock, time::Instant};

use binrw::BinRead;
use glam::{vec2, vec4, Affine2, Vec2};
use rust_shared::{console_log, countdown::CountdownDigitSets, gd::object::GDObject, util::random};

use crate::state::State;

use super::{billy::Billy, draw_obj_simple};

pub static COUNTDOWN_DIGITS: LazyLock<CountdownDigitSets> = LazyLock::new(|| {
    let bytes = include_bytes!("../../countdown_digits");

    CountdownDigitSets::read(&mut Cursor::new(bytes)).unwrap()
});

fn now() -> f64 {
    rust_shared::util::now() / 1000.0
}
const SETS_SIZE: usize = 8; // change this to 4 to get big error
pub struct Countdown {
    pub digits: [CountdownDigit; 8],
    pub state: [Option<u8>; 8],
    pub sets: [usize; SETS_SIZE],
}

impl Countdown {
    const OFFSET: Vec2 = vec2(450.0, 450.0 + 30.0 * 14.0);

    pub fn new() -> Self {
        Self {
            digits: array::from_fn(|_| CountdownDigit::new()),
            state: [None; 8],
            sets: [0; SETS_SIZE], //array::from_fn(|_| (random() * COUNTDOWN_DIGITS.0.len() as f64) as usize),
        }
    }
    pub fn update_state(&mut self, event_elapsed: f64) {
        let time_until = -event_elapsed / 1000.0;

        let days = (time_until / 86400.0).floor();
        let hours = ((time_until - (days * 86400.0)) / 3600.0).floor();
        let minutes = ((time_until - (days * 86400.0) - (hours * 3600.0)) / 60.0).floor();
        let seconds = (time_until - (days * 86400.0) - (hours * 3600.0) - (minutes * 60.0)).floor();

        fn digits(num: u8, display_zero: bool) -> (Option<u8>, Option<u8>) {
            if num == 0 && !display_zero {
                (None, None)
            } else {
                (Some(num / 10), Some(num % 10))
            }
        }

        let (dayd1, dayd2) = digits(days as u8, false);
        let (hourd1, hourd2) = digits(hours as u8, true);
        let (mind1, mind2) = digits(minutes as u8, true);
        let (secd1, secd2) = digits(seconds as u8, true);

        let state = [dayd1, dayd2, hourd1, hourd2, mind1, mind2, secd1, secd2];

        for i in 0..8 {
            if self.state[i] != state[i] {
                // let num_sets = COUNTDOWN_DIGITS.0.len();
                // let new_set = if random() < 0.2 {
                //     (random() * num_sets as f64) as usize
                // } else {
                //     self.sets[i]
                // };
                let new_set = 0;
                self.digits[i].transition_between(
                    self.state[i],
                    state[i],
                    self.sets[0],
                    self.sets[0],
                    0.8,
                );
                self.state[i] = state[i];
                self.sets[i] = new_set;
            }
        }
    }

    pub fn draw(&self, billy: &mut Billy) {
        billy.translate(Self::OFFSET);

        for (i, digit) in self.digits.iter().enumerate() {
            if i == 2 {
                billy.translate(vec2(-30.0 * 14.0, -30.0 * 14.0)); // line break
            } else if i != 0 && i % 2 == 0 {
                billy.translate(vec2(30.0 * 3.0, 0.0)); // colons
            }
            digit.draw(billy);
            billy.translate(vec2(30.0 * 7.0, 0.0));
        }
    }
}

fn draw_obj(obj: &GDObject, billy: &mut Billy) {
    // detail
    draw_obj_simple(
        billy,
        &obj,
        true,
        vec4(
            obj.detail_color.r as f32 / 255.0,
            obj.detail_color.g as f32 / 255.0,
            obj.detail_color.b as f32 / 255.0,
            obj.detail_color.opacity as f32 / 255.0,
        ),
        obj.detail_color.blending,
    );
    // main
    draw_obj_simple(
        billy,
        &obj,
        false,
        vec4(
            obj.main_color.r as f32 / 255.0,
            obj.main_color.g as f32 / 255.0,
            obj.main_color.b as f32 / 255.0,
            obj.main_color.opacity as f32 / 255.0,
        ),
        obj.main_color.blending,
    );
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
    fn from_set(set: usize, digit: u8) -> Self {
        let mut empty = Self::new();
        empty.set_to(set, digit, 0.5);
        empty
    }

    fn draw(&self, billy: &mut Billy) {
        for obj in &self.objects {
            obj.get().inspect(|o| draw_obj(o, billy));
        }
    }

    fn get_set(set: usize, digit: u8) -> &'static [GDObject] {
        &COUNTDOWN_DIGITS.0[set].0[digit as usize].objs
    }

    fn set_to(&mut self, set: usize, digit: u8, duration: f64) {
        self.objects = Self::get_set(set, digit)
            .iter()
            .map(|o| TransitioningObject::new(AnimType::Appear(*o), duration))
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
        duration: f64,
    ) {
        self.objects.clear(); // so it dont reallocate :)

        let setup = |digit: Option<u8>, set: usize| {
            digit
                .map(|d| Self::get_set(set, d))
                .unwrap_or(&[])
                .into_iter()
                .copied()
                .map(|o| (o, true))
                .collect::<Vec<_>>()
        };

        let mut trans_out = setup(digit1, set1);
        let mut trans_in = setup(digit2, set2);

        // transitions (WIP)
        for obj_out in &mut trans_out {
            for obj_in in &mut trans_in {
                if is_equivalent(obj_out.0, obj_in.0) {
                    obj_out.1 = false;
                    obj_in.1 = false;
                    self.objects.push(TransitioningObject::new(
                        AnimType::Static(obj_out.0),
                        duration,
                    ));
                }
                // if obj_out.0.id == obj_in.0.id {
                //     //  ...
                // }
            }
        }

        self.objects.extend(trans_out.iter().filter_map(|o| {
            o.1.then(|| TransitioningObject::new(AnimType::Disappear(o.0), duration))
        }));

        self.objects.extend(trans_in.iter().filter_map(|o| {
            o.1.then(|| TransitioningObject::new(AnimType::Appear(o.0), duration))
        }));

        // resort (dw this only happens once a second to like 200 objects at a time)
        self.objects.sort_by(|a, b| {
            let a = a.typ.repr_obj();
            let b = b.typ.repr_obj();
            // place blending objects at bottom of layer (dont care about detail color for this)
            let a_z = if a.main_color.blending {
                i8::MIN
            } else {
                a.z_order
            };
            let b_z = if b.main_color.blending {
                i8::MIN
            } else {
                b.z_order
            };
            (a.z_layer as u8)
                .cmp(&(b.z_layer as u8))
                .then(a_z.cmp(&b_z))
        });
    }
}

pub fn is_equivalent(obj1: GDObject, obj2: GDObject) -> bool {
    if obj1.id != obj2.id {
        return false;
    }
    let same_pos = (obj1.x - obj2.x).abs() < 0.01 && (obj1.y - obj2.y).abs() < 0.01;
    let same_transform = (obj1.ix - obj2.ix).abs() < 0.001
        && (obj1.iy - obj2.iy).abs() < 0.001
        && (obj1.jx - obj2.jx).abs() < 0.001
        && (obj1.jy - obj2.jy).abs() < 0.001;

    let same_colors = obj1.main_color == obj2.main_color && obj1.detail_color == obj2.detail_color;

    same_pos && same_transform && same_colors
}

enum AnimType {
    Appear(GDObject),
    Disappear(GDObject),
    Transition(GDObject, GDObject),
    Static(GDObject),
}

impl AnimType {
    fn repr_obj(&self) -> GDObject {
        *match self {
            AnimType::Appear(obj) => obj,
            AnimType::Disappear(obj) => obj,
            AnimType::Transition(obj, _) => obj,
            AnimType::Static(obj) => obj,
        }
    }
}

struct TransitioningObject {
    typ: AnimType,
    start_time: f64,
    duration: f64,
}

impl TransitioningObject {
    fn get(&self) -> Option<GDObject> {
        let time = now();
        let d = if time <= self.start_time {
            0.0
        } else if time > self.start_time && time < self.start_time + self.duration {
            (time - self.start_time) / self.duration
        } else {
            1.0
        };

        // WIP
        match self.typ {
            AnimType::Static(obj) => Some(obj),
            AnimType::Appear(obj) => {
                if d < 0.3 {
                    None
                } else if d < 0.6 {
                    Some(obj.tint(0.0, 1.0, 0.0, 1.0))
                } else {
                    Some(obj)
                }
            }
            AnimType::Disappear(obj) => {
                if d < 0.4 {
                    Some(obj)
                } else if d < 0.7 {
                    Some(obj.tint(0.0, 1.0, 0.0, 1.0))
                } else {
                    None
                }
            }
            AnimType::Transition(gdobject, gdobject1) => todo!(),
        }
    }

    fn new(typ: AnimType, duration: f64) -> TransitioningObject {
        let time = now();
        let delay = typ.repr_obj().y / 300.0 * 0.2;
        TransitioningObject {
            typ,
            start_time: time + random() * duration * 0.2 + delay as f64,
            duration: duration * 0.8,
        }
    }
}
