use core::fmt;
use std::str::FromStr;

use crate::{error::ParseWavelengthError, frequency::Frequency, C};

#[derive(Debug, Clone, Copy)]
pub struct Meters(pub f64);

impl Meters {
    pub fn to_frequency(self) -> Frequency {
        match C / self.0.abs() {
            mhz @ 1_000_000.0.. => Frequency::MHz(mhz / 1_000_000.0),
            khz @ 1_000.0.. => Frequency::KHz(khz / 1_000.0),
            hz => Frequency::Hz(hz),
        }
    }
}

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.04} meters", self.0)
    }
}

impl FromStr for Meters {
    type Err = ParseWavelengthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s
            .find(|u: char| u.is_ascii_alphabetic())
            .ok_or(ParseWavelengthError::BadSuffix)?;
        let (left, right) = s.split_at(mid);

        match right.to_ascii_lowercase().as_str() {
            "m" => Ok(Meters(left.parse()?)),
            "cm" => Ok(Meters(left.parse::<f64>()? / 100.0)),
            _ => Err(ParseWavelengthError::BadSuffix),
        }
    }
}
