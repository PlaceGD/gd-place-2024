use std::{
    io::{stdin, BufRead},
    net::TcpListener,
    sync::{mpsc::Sender, Arc},
    thread,
};

use owo_colors::OwoColorize;
use parking_lot::FairMutex;

use crate::AppData;

pub const EVENT_START: i64 = 1731697200000;

pub fn errmsg(s: impl AsRef<str>) {
    eprintln!("{}", s.as_ref().bright_red().bold());
}
pub fn warnmsg(s: impl AsRef<str>) {
    eprintln!("{}", s.as_ref().bright_yellow().dimmed());
}

pub fn parse_time(s: &str) -> Result<i64, String> {
    let initial_s = s;
    let (from_start, s) = if let Some(s) = s.strip_prefix('$') {
        (true, s)
    } else if !s.starts_with(|c: char| c.is_ascii_digit()) {
        return Err(format!("invalid prefix for time literal `{}`", initial_s));
    } else {
        (false, s)
    };

    enum Format {
        Secs,
        Milis,
        Mins,
        Hours,
        Days,
    }

    let (format, s) = if let Some(s) = s.strip_suffix("ms") {
        (Format::Milis, s)
    } else if let Some(s) = s.strip_suffix("s") {
        (Format::Secs, s)
    } else if let Some(s) = s.strip_suffix("m") {
        (Format::Mins, s)
    } else if let Some(s) = s.strip_suffix("h") {
        (Format::Hours, s)
    } else if let Some(s) = s.strip_suffix("d") {
        (Format::Days, s)
    } else if s.ends_with(|c: char| c.is_ascii_digit()) {
        // warnmsg(format!(
        //     "no suffix for time literal `{}`, defaulting to milis",
        //     initial_s
        // ));
        (Format::Milis, s)
    } else {
        return Err(format!("invalid suffix for time literal `{}`", initial_s));
    };
    let num = s
        .parse::<i64>()
        .map_err(|_| format!("malformed number in time literal `{}`", initial_s))?;
    let milis = match format {
        Format::Secs => num * 1000,
        Format::Milis => num,
        Format::Mins => num * 1000 * 60,
        Format::Hours => num * 1000 * 60 * 60,
        Format::Days => num * 1000 * 60 * 60 * 24,
    };

    Ok(milis + if from_start { EVENT_START } else { 0 })
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Goto(i64),
    GotoAnim { to: i64, duration: f32 },
    Control(f32),
    // StopControl,
}

pub fn control(send: Sender<Message>) {
    loop {
        let mut read = String::new();
        stdin().lock().read_line(&mut read).unwrap();
        let read = read.trim();

        let mut pieces = read.split(" ").filter(|v| !v.is_empty());
        let Some(cmd) = pieces.next() else {
            errmsg("no command provided");
            continue;
        };
        match cmd {
            "goto" => {
                let Some(time) = pieces.next() else {
                    errmsg("no time provided");
                    continue;
                };
                let time = match parse_time(time) {
                    Ok(t) => t,
                    Err(msg) => {
                        errmsg(msg);
                        continue;
                    }
                };
                if let Some(duration) = pieces.next() {
                    let duration = match parse_time(duration) {
                        Ok(t) => t,
                        Err(msg) => {
                            errmsg(msg);
                            continue;
                        }
                    };
                    send.send(Message::GotoAnim {
                        to: time,
                        duration: duration as f32,
                    })
                    .unwrap();
                } else {
                    send.send(Message::Goto(time)).unwrap();
                }
            }
            "control" => {
                let Some(multiplier) = pieces.next() else {
                    errmsg("no multiplier provided");
                    continue;
                };

                let multiplier = match multiplier.parse::<f32>() {
                    Ok(t) => t,
                    Err(_) => {
                        errmsg(format!("malformed number in multiplier `{}`", multiplier));
                        continue;
                    }
                };
                send.send(Message::Control(multiplier)).unwrap();
            }
            _ => {
                errmsg(format!("invalid command `{}`", cmd));
            }
        }
    }
}
