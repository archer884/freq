use core::fmt;
use std::str::FromStr;

use crate::{error::ParseFrequencyError, wavelength::Meters, C};

#[derive(Debug, Clone, Copy)]
pub enum Frequency {
    MHz(f64),
    KHz(f64),
    Hz(f64),
}

impl Frequency {
    pub fn to_wavelength(self) -> Meters {
        Meters(C / self.normalize())
    }

    fn normalize(self) -> f64 {
        match self {
            Frequency::MHz(hz) => hz * 1_000_000.0,
            Frequency::KHz(hz) => hz * 1_000.0,
            Frequency::Hz(hz) => hz,
        }
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Frequency::MHz(n) => write!(f, "{n:.05}MHz"),
            Frequency::KHz(n) => write!(f, "{n:.05}KHz"),
            Frequency::Hz(n) => write!(f, "{n:.05}Hz"),
        }
    }
}

impl FromStr for Frequency {
    type Err = ParseFrequencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s
            .find(|u: char| u.is_ascii_alphabetic())
            .ok_or(ParseFrequencyError::BadSuffix)?;
        let (left, right) = s.split_at(mid);

        match right.to_ascii_lowercase().as_str() {
            "mhz" => Ok(Frequency::MHz(left.parse()?)),
            "khz" => Ok(Frequency::KHz(left.parse()?)),
            "hz" => Ok(Frequency::Hz(left.parse()?)),
            _ => Err(ParseFrequencyError::BadSuffix),
        }
    }
}
