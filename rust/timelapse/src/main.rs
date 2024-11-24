//! all the shit around here is coded like a poopy poopo stinky a boobo vgagaga cause i just wanted to get it working

mod control;

use std::collections::{BTreeMap, HashMap};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;

use control::{control, Message, EVENT_START};
use draw_lib::{create_view, GDObjectOpt, State};
use glam::{vec2, Vec2};
use num_bigint::BigUint;
use parking_lot::FairMutex;
use rust_shared::util::quick_image_load;
use serde_json::Value;
use winit::application::ApplicationHandler;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

// welcome back
pub fn baseconvert(digits: &[u8], from_base: u32, to_base: u32) -> Vec<u8> {
    // ignore how ass everything is

    let big_from_base = BigUint::from(from_base);
    let big_to_base = BigUint::from(to_base);
    let mut big_sum = BigUint::from(0u8);

    let mut zeroes = 0;
    for i in 0..digits.len() {
        if digits[i] == 0 {
            zeroes += 1;
        } else {
            break;
        }
    }

    for i in 0..digits.len() {
        let p = digits.len() - 1 - i;
        big_sum += digits[i] as u32 * big_from_base.pow(p as u32);
    }

    use num::NumCast;

    let mut ret: Vec<u8> = vec![];
    while big_sum > BigUint::from(0u8) {
        let digit = big_sum.clone() % &big_to_base;
        ret.push(NumCast::from(digit).unwrap());
        big_sum /= &big_to_base;
    }
    ret.extend(std::iter::repeat(0).take(zeroes));
    ret.reverse();
    ret
}
fn decode_string(s: &str, base: u32) -> Vec<u8> {
    baseconvert(s.as_bytes(), base, 256)
}

#[derive(Debug, Clone, Copy)]
enum HistoryItem {
    Place {
        key: [u8; 20],
        obj: GDObjectOpt,
    },
    Delete {
        key: [u8; 20],
        obj: GDObjectOpt, // this is here so that when going backwards it adds the object
    },
}
type History = BTreeMap<i64, HistoryItem>;

struct GotoAnimInfo {
    to: i64,

    duration: f32,

    started_time: i64,
    started: Instant,
}

struct AppData {
    window: Arc<Window>,
    state: State,

    last_frame: Instant,

    history: History,

    current_time: i64,

    rec: Receiver<Message>,

    mouse_pos: Vec2,
    dragging: Option<(Vec2, Vec2)>,

    anim: Option<GotoAnimInfo>,

    control_mult: f32,
    left_pressed: bool,
    right_pressed: bool,
    // controlling: Option<f32>,
}

#[derive(Default)]
struct App {
    data: Option<AppData>,
}
fn apply_history_range(history: &History, state: &mut State, from: i64, to: i64) {
    let apply = |item: HistoryItem, forwards: bool, state: &mut State| match (item, forwards) {
        (HistoryItem::Place { key, obj }, true) | (HistoryItem::Delete { key, obj }, false) => {
            state.add_object_u8k(key, obj);
        }
        (HistoryItem::Delete { key, obj }, true) | (HistoryItem::Place { key, obj }, false) => {
            state.delete_object_u8k(key);
        }
    };
    if from <= to {
        for (_, &v) in history.range(from..=to) {
            apply(v, true, state);
        }
    } else {
        for (_, &v) in history.range(to..=from).rev() {
            apply(v, false, state);
        }
    }

    // let range = if from <= to { from..=to } else { (from..=to).rev() };
}

// fn lol(v: Vec<i32>)

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let spritesheet = quick_image_load(include_bytes!("../assets/spritesheet.png"));

        let db = serde_json::from_slice::<Value>(&std::fs::read("../../db.json").unwrap()).unwrap();
        let mut all_objs = HashMap::new();
        let mut history = BTreeMap::new();
        for (_, v) in db["history"].as_object().unwrap() {
            let key: [u8; 20] = v["objKey"].as_str().unwrap().as_bytes().try_into().unwrap();
            let time = v["time"].as_i64().unwrap();
            if v["chunk"].is_null() {
                let obj =
                    GDObjectOpt::from_bytes(decode_string(v["object"].as_str().unwrap(), 126))
                        .unwrap();
                all_objs.insert(key, obj);
                history.insert(time, HistoryItem::Place { key, obj });
            } else {
                history.insert(
                    time,
                    HistoryItem::Delete {
                        key,
                        obj: all_objs[&key],
                    },
                );
            }
        }

        let (send, rec) = mpsc::channel();
        let mut data = AppData {
            state: pollster::block_on(create_view(
                window.clone(),
                spritesheet.as_bytes(),
                spritesheet.width(),
                spritesheet.height(),
            ))
            .unwrap(),
            window,
            history,
            rec,
            current_time: EVENT_START,
            mouse_pos: Vec2::ZERO,
            dragging: None,
            anim: None,
            last_frame: Instant::now(),

            control_mult: 1.0,
            left_pressed: false,
            right_pressed: false,
            // controlling: None
        };

        // println!("adding");
        // for (_, v) in db["objects"].as_object().unwrap() {
        //     for (k, v) in v.as_object().unwrap() {
        //         let v = v.as_str().unwrap();
        //         // println!("{:?}", v);
        //         let obj = GDObjectOpt::from_bytes(decode_string(v, 126)).unwrap();
        //         data.state.add_object(k.into(), obj).unwrap();
        //     }
        // }

        data.state.set_bg_color(4, 24, 46);
        data.state.set_ground1_color(5, 40, 77);
        data.state.set_ground2_color(0, 82, 165);

        self.data = Some(data);

        thread::spawn(move || {
            control(send);
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(data) = &mut self.data else {
            return;
        };

        if let Some(anim) = &mut data.anim {
            let now = Instant::now();

            let elapsed = (now - anim.started).as_millis() as f32;

            let to = anim.started_time
                + ((anim.to - anim.started_time) as f32 * (elapsed / anim.duration).min(1.0))
                    as i64;

            apply_history_range(&data.history, &mut data.state, data.current_time, to);
            data.current_time = to;

            // println!("fps {}", 1.0 / (elapsed / 1000.0));

            if elapsed >= anim.duration {
                data.anim = None;
            }
        }
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(to) => {
                data.state.resize(to.width, to.height);
            }
            WindowEvent::RedrawRequested => {
                for i in data.rec.try_iter() {
                    match i {
                        Message::Goto(to) => {
                            apply_history_range(
                                &data.history,
                                &mut data.state,
                                data.current_time,
                                to,
                            );
                            data.current_time = to;
                        }
                        Message::GotoAnim { to, duration: over } => {
                            data.anim = Some(GotoAnimInfo {
                                to,
                                duration: over,

                                started_time: data.current_time,
                                started: Instant::now(),
                            });
                        }
                        Message::Control(mult) => data.control_mult = mult,
                    }
                }

                let now = Instant::now();
                let delta = (now - data.last_frame).as_secs_f32();
                data.last_frame = now;
                // println!("fps {}", 1.0 / delta);

                if data.left_pressed {
                    let to = data.current_time - (delta * 1000.0 * data.control_mult) as i64;
                    apply_history_range(&data.history, &mut data.state, data.current_time, to);
                    data.current_time = to;
                }
                if data.right_pressed {
                    let to = data.current_time + (delta * 1000.0 * data.control_mult) as i64;
                    apply_history_range(&data.history, &mut data.state, data.current_time, to);
                    data.current_time = to;
                }

                data.state.render(0.0);
                data.window.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                data.mouse_pos = vec2(position.x as f32, position.y as f32);
                if let Some((prev_mouse, prev_camera)) = data.dragging {
                    data.state.camera_pos = prev_camera
                        + (data.mouse_pos - prev_mouse) * vec2(-1.0, 1.0)
                            / data.state.get_zoom_scale();
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                //
                match (button, state.is_pressed()) {
                    (winit::event::MouseButton::Left, true) => {
                        data.dragging = Some((data.mouse_pos, data.state.camera_pos))
                    }
                    (winit::event::MouseButton::Left, false) => data.dragging = None,
                    _ => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::LineDelta(_, y) = delta {
                    data.state.zoom += y / y.abs();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                match (event.physical_key, event.state.is_pressed()) {
                    (PhysicalKey::Code(KeyCode::ArrowLeft), p) => data.left_pressed = p,
                    (PhysicalKey::Code(KeyCode::ArrowRight), p) => data.right_pressed = p,
                    _ => {}
                }
            }
            _ => (),
        }
    }
}
fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
