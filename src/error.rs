use core::fmt;
use std::{error, num::ParseFloatError};

#[derive(Debug)]
pub enum ParseFrequencyError {
    BadSuffix,
    Float(ParseFloatError),
}

impl From<ParseFloatError> for ParseFrequencyError {
    fn from(e: ParseFloatError) -> Self {
        Self::Float(e)
    }
}

impl fmt::Display for ParseFrequencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseFrequencyError::BadSuffix => f.write_str("bad suffix -- use MHz, KHz, Hz"),
            ParseFrequencyError::Float(e) => e.fmt(f),
        }
    }
}

impl error::Error for ParseFrequencyError {}

#[derive(Debug)]
pub enum ParseWavelengthError {
    BadSuffix,
    Float(ParseFloatError),
}

impl From<ParseFloatError> for ParseWavelengthError {
    fn from(e: ParseFloatError) -> Self {
        Self::Float(e)
    }
}

impl fmt::Display for ParseWavelengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseWavelengthError::BadSuffix => f.write_str("bad suffix -- use m, cm"),
            ParseWavelengthError::Float(e) => e.fmt(f),
        }
    }
}

impl error::Error for ParseWavelengthError {}
