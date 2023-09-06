use std::num::{ParseIntError,ParseFloatError};
use std::str::Utf8Error;

use thiserror::Error;

/// Represents the possible errors that can occur during parsing of a `StandardForm` number.
#[derive(Error,Debug,Clone)]
pub enum ParsingStandardFormError {
    /// Error that occurs while parsing the mantissa as a `ParseFloatError`.
    #[error("Failed parsing mantissa due to {0}")]
    Mantissa(#[from] ParseFloatError),
    /// Error that occurs while parsing the exponent as a `ParseIntError`.
    #[error("Failed parsing exponent due to {0}")]
    Exponent(#[from] ParseIntError),
    /// Indicates an invalid format that doesn't match any valid `StandardForm` notation.
    #[error("Invalid format")]
    InvalidFormat,
    /// Only occurs when `StandardFrom::try_from(&[u8])` is done 
    #[error("Given bytes are not formatted in UTF-8")]
    InvalidBytes(#[from] Utf8Error),

    /// Only occurs when `num` feature is enabled and `from_str_radix` method is used
    #[cfg(feature = "num")]
    #[error("Invalid radix : Only radix 10 is supported")]
    InvalidRadix
}