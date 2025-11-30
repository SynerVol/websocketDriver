// src/command.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    GoTo {
        lat: f64,
        lon: f64,
        alt_m: f32,
        #[serde(default)]
        speed_mps: Option<f32>,
    },
    TakePicture {
        #[serde(default)]
        mode: Option<String>, // e.g., "photo" or "hdr"
    },
    RotateAndFilm {
        degrees: u16, // expect 360 typically
        #[serde(default)]
        duration_s: Option<u16>,
        #[serde(default)]
        quality: Option<String>, // e.g., "1080p60"
    },
    Ping,
}

impl Command {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Command::GoTo { lat, lon, alt_m, speed_mps } => {
                if !(-90.0..=90.0).contains(lat) {
                    return Err("Latitude must be in [-90, 90]".into());
                }
                if !(-180.0..=180.0).contains(lon) {
                    return Err("Longitude must be in [-180, 180]".into());
                }
                if *alt_m < 0.0 {
                    return Err("Altitude must be non-negative".into());
                }
                if let Some(s) = speed_mps {
                    if *s <= 0.0 || *s > 50.0 {
                        return Err("Speed must be in (0, 50] m/s".into());
                    }
                }
                Ok(())
            }
            Command::TakePicture { .. } => Ok(()),
            Command::RotateAndFilm { degrees, duration_s, .. } => {
                if *degrees == 0 || *degrees > 1080 {
                    return Err("Degrees must be in [1, 1080]".into());
                }
                if let Some(d) = duration_s {
                    if *d == 0 || *d > 600 {
                        return Err("Duration must be in [1, 600] seconds".into());
                    }
                }
                Ok(())
            }
            Command::Ping => Ok(()),
        }
    }
}
