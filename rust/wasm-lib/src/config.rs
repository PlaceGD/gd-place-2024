use std::{
    fs,
    path::{Path, PathBuf},
};

use rust_shared::countdown::DIGIT_SETS;
use serde::{Deserialize, Serialize};

use crate::{error::AppError, App};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub general: General,
    pub clock: Clock,
    pub grid: Grid,
    pub sets: Sets,
    pub background: Background,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct General {
    pub zoom: f32,
    pub fps_cap: usize,
    // pub quality: f32, // 0 to 1
    pub rng_seed: Option<u64>,
}

impl Default for General {
    fn default() -> Self {
        Self {
            zoom: -6.0,
            fps_cap: 30,
            rng_seed: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClockPadding {
    pub top: isize,
    pub left: isize,
    pub bottom: isize,
    pub right: isize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Clock {
    pub position: String,
    pub padding: ClockPadding,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            position: "center".into(),
            padding: ClockPadding {
                top: 0,
                left: 0,
                bottom: 0,
                right: 0,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Grid {
    pub opacity: f32,
}

impl Default for Grid {
    fn default() -> Self {
        Self { opacity: 100.0 }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SetsDigits {
    pub hours: usize,
    pub minutes: usize,
    pub seconds: usize,
    pub colonh: usize,
    pub colonm: usize,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Sets {
    pub colon_sets: Vec<usize>,
    pub digit_sets: Vec<usize>,
    pub sets: Option<SetsDigits>,
    pub show_colons: bool,
    pub digit_change_frequency: usize,
}

impl Default for Sets {
    fn default() -> Self {
        Self {
            colon_sets: vec![0, 1, 2, 3, 4],
            digit_sets: (0..DIGIT_SETS).collect(),
            sets: None,
            show_colons: true,
            digit_change_frequency: 1200,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BackgroundColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Background {
    pub color: BackgroundColor,
    pub image: String,
}

impl Default for Background {
    fn default() -> Self {
        Self {
            color: BackgroundColor { r: 4, g: 24, b: 46 },
            image: "./background.png".into(),
        }
    }
}

const CONFIG_DETAILS: &'static str = "

";

impl Config {
    fn get_and_write_default(path: &Path) -> Result<Self, AppError> {
        let config = Self::default();

        let config_str = toml::to_string(&config).unwrap();

        let _ = fs::write(path, format!("{config_str}\n{CONFIG_DETAILS}"))
            .map_err(AppError::ConfigWriteFailed)?;

        Ok(config)
    }

    pub fn from_file_or_default() -> Result<Self, AppError> {
        let path: PathBuf = "./config.toml".into();

        if let Ok(config_str) = fs::read_to_string(&path) {
            Ok(toml::from_str(&config_str).map_err(AppError::ConfigReadFailed)?)
        } else {
            Self::get_and_write_default(&path)
        }
    }
}
