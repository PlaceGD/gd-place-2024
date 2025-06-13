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
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Background {
    pub color: Color,
    pub image_tint: Color,
    pub image: String,
    pub fit: String,
}

impl Default for Background {
    fn default() -> Self {
        Self {
            color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            image_tint: Color {
                r: 4,
                g: 24,
                b: 46,
                a: 100,
            },
            image: "./background.png".into(),
            fit: "tile".into(),
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

    fn validate_config(&self) -> Result<(), AppError> {
        match &self.clock.position[..] {
            "center" | "top-left" | "top-right" | "bottom-left" | "bottom-right" => (),
            value => return Err(AppError::ConfigValidationError {
                reason: format!("unknown value `{value}` in `clock.postion`! must be one of `center, top-left, top-right, bottom-left, bottom-right`") 
            })
        }

        match &self.background.fit[..] {
            "fill" | "cover" | "contain" | "tile" | "none" => (),
            value => return Err(AppError::ConfigValidationError {
                reason: format!("unknown value `{value}` in `background.tilin`! must be one of `fill, cover, contain, tile, none`") 
            })
        }

        match self.grid.opacity {
            0.0..=100.0 => (),
            value => {
                return Err(AppError::ConfigValidationError {
                    reason: format!(
                        "`{value}` outside range for `grid.opacity`! must be between 0.0 and 100.0"
                    ),
                })
            }
        }

        let is_valid_colon = |c: &usize| (0..5).contains(c);
        let is_valid_digit = |d: &usize| (0..DIGIT_SETS).contains(d);

        let invalid_colons = self.sets.colon_sets.iter().any(|c| !is_valid_colon(&c));

        if invalid_colons {
            let joined = self
                .sets
                .colon_sets
                .iter()
                .filter(|c| !is_valid_colon(c))
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            return Err(AppError::ConfigValidationError {
                reason: format!(
                    "invalid colon sets `{}` found in `sets.colon_sets`! must be between 0 and 4",
                    joined
                ),
            });
        }

        let invalid_digits = self.sets.digit_sets.iter().any(|d| !is_valid_digit(d));

        if invalid_digits {
            let joined = self
                .sets
                .digit_sets
                .iter()
                .filter(|d| !is_valid_digit(d))
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            return Err(AppError::ConfigValidationError {
                reason: format!(
                    "invalid digit sets `{}` found in `sets.digit_sets`! must be between 0 and {}",
                    joined,
                    DIGIT_SETS - 1
                ),
            });
        }

        if let Some(sets) = &self.sets.sets {
            if !is_valid_digit(&sets.hours) {
                return Err(AppError::ConfigValidationError {
                    reason: format!(
                        "invalid hour digit `{}` found in `sets.sets`! must be between 0 and {}",
                        sets.hours,
                        DIGIT_SETS - 1
                    ),
                });
            }
            if !is_valid_digit(&sets.minutes) {
                return Err(AppError::ConfigValidationError {
                    reason: format!(
                        "invalid minute digit `{}` found in `sets.sets`! must be between 0 and {}",
                        sets.minutes,
                        DIGIT_SETS - 1
                    ),
                });
            }
            if !is_valid_digit(&sets.seconds) {
                return Err(AppError::ConfigValidationError {
                    reason: format!(
                        "invalid second digit `{}` found in `sets.sets`! must be between 0 and {}",
                        sets.seconds,
                        DIGIT_SETS - 1
                    ),
                });
            }

            if !is_valid_colon(&sets.colonh) {
                return Err(AppError::ConfigValidationError {
                    reason: format!(
                        "invalid hour colon `{}` found in `sets.sets`! must be between 0 and 4",
                        sets.colonh,
                    ),
                });
            }
            if !is_valid_colon(&sets.colonm) {
                return Err(AppError::ConfigValidationError {
                    reason: format!(
                        "invalid minute colon `{}` found in `sets.sets`! must be between 0 and 4",
                        sets.colonh,
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn from_file_or_default() -> Result<Self, AppError> {
        let path: PathBuf = "./config.toml".into();

        let config = if let Ok(config_str) = fs::read_to_string(&path) {
            toml::from_str(&config_str).map_err(AppError::ConfigReadFailed)?
        } else {
            Self::get_and_write_default(&path)?
        };

        config.validate_config()?;

        Ok(config)
    }
}
