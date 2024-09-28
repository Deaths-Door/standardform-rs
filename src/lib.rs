#![doc = include_str!("../README.md")]
#![deny(
    missing_docs,
    missing_debug_implementations,
    unused_mut,
    unused_allocation,
    unused_must_use,
    unreachable_patterns,
    unstable_features,
    trivial_casts,
    overflowing_literals,
    clippy::cargo,
    clippy::perf,
    clippy::complexity,
    clippy::style,
    clippy::suspicious
)]
#![cfg_attr(
    not(feature = "python"),
    forbid(non_snake_case, unsafe_op_in_unsafe_fn, unsafe_code)
)]
#![cfg_attr(
    feature = "python",
    allow(non_snake_case, unsafe_op_in_unsafe_fn, unsafe_code)
)]

#[cfg(all(feature = "nom", not(feature = "_bindings")))]
mod nom;

#[cfg(all(feature = "nom", not(feature = "_bindings")))]
pub use nom::*;

#[cfg(feature = "python")]
mod python;

#[cfg(feature = "wasm")]
mod wasm;

use core::f64;
use core::num::{ParseFloatError, ParseIntError};
use core::str::Utf8Error;

use decorum::Finite;
use num_traits::real::Real;
use num_traits::{One, Zero};

use thiserror::Error;

/// Represents a number in standard form.
///
/// The `Standardform` struct holds the significand (mantissa) of the number
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
#[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StandardForm {
    mantissa: Finite<f64>,
    exponent: i8,
}

impl StandardForm {
    /// Creates a new instance of `StandardForm` with the given mantissa and exponent.
    ///
    /// This constructor initializes a new `StandardForm` instance with the provided `mantissa` and `exponent`.
    /// It's important to note that the provided `mantissa` and `exponent` may not be exactly the same as the
    /// values stored in the resulting instance. The values are adjusted automatically to adhere to the rules
    /// of standard form representation, ensuring the most appropriate form for the given input.
    ///
    ///  ## Rules :
    /// If the current mantissa and exponent do not satisfy the standard form representation requirements,
    /// this method will adjust them while maintaining the value of the number represented. The adjustment
    /// ensures that the mantissa is between 1 (inclusive) and 10 (exclusive) and the exponent is such that
    /// the product of mantissa and 10 raised to the exponent yields the original number.
    pub fn new(mantissa: Finite<f64>, exponent: i8) -> Self {
        let mut instance = Self::new_unchecked(mantissa, exponent);
        instance.adjust();
        instance
    }

    /// Returns the 'real' value of the struct
    pub fn as_finite(self) -> Finite<f64> {
        self.mantissa * 10f64.powi(self.exponent.into())
    }

    pub(crate) const fn new_unchecked(mantissa: Finite<f64>, exponent: i8) -> Self {
        Self { mantissa, exponent }
    }

    fn in_range(&self) -> bool {
        // https://en.wikipedia.org/wiki/Standard_form
        // 1 ≤ a < 10 -- Although I include negative numbers as well

        let abs = self.mantissa.abs();

        // 1.0 <= abs is same as abs >= 1.0
        // aka

        #[allow(clippy::manual_range_contains)]
        {
            abs >= 1.0 && abs < 10.0
        }
    }

    fn adjust(&mut self) {
        if self.in_range() || self.mantissa == 0.0 {
            return;
        }

        use num_traits::real::Real;

        let log10_mantissa = self.mantissa.abs().log10();

        // Find the largest power of 10
        let adjustment = log10_mantissa.floor();

        if adjustment != 0.0 {
            self.mantissa /= Finite::from_inner(10f64).powf(adjustment);
            self.exponent += adjustment.into_inner() as i8;
        }

        // Final adjustment if it's still slightly out of range
        if !self.in_range() {
            if self.mantissa > -1.0 && self.mantissa < 1.0 {
                self.mantissa *= 10.0;
                self.exponent -= 1;
            } else {
                self.mantissa /= 10.0;
                self.exponent += 1;
            }
        }
    }
}

impl Default for StandardForm {
    #[must_use]
    fn default() -> Self {
        Self::one()
    }
}

impl core::fmt::Display for StandardForm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.exponent > 4 {
            return write!(f, "{}", self.to_scientific_notation());
        };

        write!(f, "{}", self.mantissa * 10_f64.powi(self.exponent as i32))
    }
}

impl StandardForm {
    /// Returns a reference to the StandardForm representing the significand (mantissa) of the number.
    #[must_use]
    pub const fn mantissa(&self) -> &Finite<f64> {
        &self.mantissa
    }

    /// Returns the exponent that determines the power of 10 by which the significand should be multiplied.
    #[must_use]
    pub const fn exponent(&self) -> &i8 {
        &self.exponent
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
impl StandardForm {
    /// Returns the string representation of the number in scientific notation.
    #[must_use]
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
    pub fn to_scientific_notation(&self) -> String {
        format!("{}e{}", self.mantissa, self.exponent)
    }

    /// Returns the string representation of the number in engineering notation.
    #[must_use]
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
    pub fn to_engineering_notation(&self) -> String {
        format!("{}*10^{}", self.mantissa, self.exponent)
    }
}

#[cfg(feature = "std")]
impl TryFrom<&str> for StandardForm {
    type Error = crate::ParsingStandardFormError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(number) = value.parse::<f64>() {
            let output: Result<Self, ConversionError> = number.try_into();
            return output.map_err(|e| e.into());
        }

        if let Some(index) = value.find('e').or_else(|| value.find('E')) {
            let m_str: Finite<f64> = value[0..index].parse()?;
            let e_str: i8 = value[index + 1..].parse()?;
            return Ok(StandardForm::new(m_str, e_str));
        }

        if let Some(index) = value.find('^') {
            let m_str: Finite<f64> = value[0..index - 3].parse()?;
            let e_str: i8 = value[index + 1..].parse()?;
            return Ok(StandardForm::new(m_str, e_str));
        }

        Err(crate::ParsingStandardFormError::InvalidFormat)
    }
}

/// Represents the possible errors that can occur during parsing of a `StandardForm` number.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
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

    /// Only occurs when from_str_radix` method is used
    #[error("Invalid radix : Only radix 10 is supported")]
    InvalidRadix,

    /// Constraints Voliated for constructing the given f64
    #[error("{0}")]
    ConstraintsVoliated(#[from] ConversionError),
}

impl core::ops::Neg for StandardForm {
    type Output = Self;
    #[must_use]
    fn neg(self) -> Self::Output {
        Self::new_unchecked(-self.mantissa, self.exponent)
    }
}
impl core::ops::Add for StandardForm {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let max_power = self.exponent.max(other.exponent);
        let num_sum = self.mantissa * 10.0_f64.powf((self.exponent - max_power) as f64)
            + other.mantissa * 10.0_f64.powf((other.exponent - max_power) as f64);
        StandardForm::new(num_sum, max_power)
    }
}

impl core::ops::AddAssign for StandardForm {
    fn add_assign(&mut self, other: Self) {
        let max_power = self.exponent.max(other.exponent);
        let num_sum = self.mantissa * 10.0_f64.powf((self.exponent - max_power) as f64)
            + other.mantissa * 10.0_f64.powf((other.exponent - max_power) as f64);

        self.mantissa = num_sum;
        self.exponent = max_power;

        self.adjust();
    }
}

pub(crate) fn round(result: Finite<f64>) -> Finite<f64> {
    const TOLERANCE: f64 = 1.0e6;
    (result * TOLERANCE).round() / TOLERANCE
}

impl core::ops::Sub for StandardForm {
    type Output = Self;
    #[must_use]
    fn sub(self, other: Self) -> Self {
        let min = self.exponent.min(other.exponent);

        let x = self.mantissa * 10_f64.powi((self.exponent - min) as i32);
        let y = other.mantissa * 10_f64.powi((other.exponent - min) as i32);

        let result = x - y;

        StandardForm::new(round(result), min)
    }
}

impl core::ops::SubAssign for StandardForm {
    fn sub_assign(&mut self, other: Self) {
        let min = self.exponent.min(other.exponent);

        let x = self.mantissa * 10_i32.pow((self.exponent - min) as u32) as f64;
        let y = other.mantissa * 10_i32.pow((other.exponent - min) as u32) as f64;

        self.mantissa = x - y;
        self.exponent = min;
        self.adjust();
    }
}

impl core::ops::Mul for StandardForm {
    type Output = Self;
    #[must_use]
    fn mul(self, other: Self) -> Self {
        let exponent = self.exponent + other.exponent;
        let mantissa = self.mantissa * other.mantissa;
        StandardForm::new(round(mantissa), exponent)
    }
}

impl core::ops::MulAssign for StandardForm {
    fn mul_assign(&mut self, other: Self) {
        let exponent = self.exponent + other.exponent;
        let mantissa = self.mantissa * other.mantissa;

        self.mantissa = round(mantissa);
        self.exponent = exponent;

        self.adjust();
    }
}

impl core::ops::Div for StandardForm {
    type Output = Self;
    #[must_use]
    fn div(self, other: Self) -> Self {
        StandardForm::new(
            self.mantissa / other.mantissa,
            self.exponent - other.exponent,
        )
    }
}

impl core::ops::DivAssign for StandardForm {
    fn div_assign(&mut self, other: Self) {
        self.mantissa /= other.mantissa;
        self.exponent /= other.exponent;
        self.adjust();
    }
}

impl core::ops::Rem for StandardForm {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        StandardForm::from(self.as_finite() % rhs.as_finite())
    }
}

impl core::ops::RemAssign for StandardForm {
    fn rem_assign(&mut self, other: StandardForm) {
        *self = self.clone() % other;
    }
}

macro_rules! primitives {
    (ints => $($t:ty),*) => {
        $(
            impl From<$t> for StandardForm {
                #[must_use]
                fn from(value: $t) -> Self {
                    StandardForm::new((value as f64).into(),0)
                }
            }

            primitives!(ops => $t);
        )*
    };

    (ops => $($t:ty),*) => {
        $(
            impl PartialEq<$t> for StandardForm {
                fn eq(&self,other: &$t) -> bool {
                    let rhs : Self = (*other).into();
                    *self == rhs
                }
            }

            impl PartialOrd<$t> for StandardForm {
                fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                    let rhs : Self = (*other).into();
                    self.partial_cmp(&rhs)
                }
            }

            impl  core::ops::Add<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn add(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self + rhs
                }
            }

            impl  core::ops::AddAssign<$t> for StandardForm {
                fn add_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self += rhs;
                }
            }

            impl  core::ops::Sub<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn sub(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self - rhs
                }
            }

            impl  core::ops::SubAssign<$t> for StandardForm {
                fn sub_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self -= rhs;
                }
            }

            impl  core::ops::Mul<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn mul(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self * rhs
                }
            }

            impl  core::ops::MulAssign<$t> for StandardForm {
                fn mul_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self *= rhs;
                }
            }

            impl core::ops:: Div<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn div(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self / rhs
                }
            }

            impl  core::ops::DivAssign<$t> for StandardForm {
                fn div_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self /= rhs;
                }
            }

            impl  core::ops::Rem<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn rem(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self % rhs
                }
            }

            impl  core::ops::RemAssign<$t> for StandardForm {
                fn rem_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self %= rhs;
                }
            }
        )*
    };
}

primitives!(ints => u8,u16,u32,u64,i8,i16,i32,i64);
primitives!(ops => Finite<f64>);

impl From<Finite<f64>> for StandardForm {
    fn from(value: Finite<f64>) -> Self {
        Self::new(value, Zero::zero())
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
/// ConversionError
pub enum ConversionError {
    /// Constraint of Finite<f64> is voliated by NaN
    #[error("Constraint of Finite<f64> is voliated by NaN")]
    NaN,

    /// Constraint of Finite<f64> is voliated by Infinity
    #[error("Constraint of Finite<f64> is voliated by Infinity")]
    Infinity,
}

impl TryFrom<f64> for StandardForm {
    type Error = ConversionError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        // Check for NaN
        if value.is_nan() {
            return Err(ConversionError::NaN);
        }

        // Check for Infinity
        if value.is_infinite() {
            return Err(ConversionError::Infinity);
        }

        Ok(Self::from(Finite::from(value)))
    }
}

impl TryFrom<f32> for StandardForm {
    type Error = ConversionError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(value as f64)
    }
}

// TODO - Trig Functions
/*macro_rules! trig_functions {
    ($( {
        $(#[$attr:meta])* $fn : ident
    })*) => {
        impl StandardForm {
            $(
                $(#[$attr])*
                #[cfg(not(feature="js"))]
                pub fn $fn <T : From<f64>>(self) -> T  {
                    f64::from(self). $fn ().into()
                }

                $(#[$attr])*
                #[cfg(feature="js")]
                #[cfg_attr(feature="js", wasm_bindgen)]
                pub fn $fn (self) -> Self  {
                    f64::from(self). $fn ().into()
                }
            )*

        }
    };
}

trig_functions!(
    { /// Computes the sine of a number (in radians).
      sin }
    { /// Computes the cosine of a number (in radians).
      cos }
    { /// Computes the tangent of a number (in radians).
     tan }
    { /// Computes the arcsine of a number.
      asin }
    { /// Computes the arccosine of a number.
      acos }
    { /// Computes the arctangent of a number.
      atan }
    { /// Computes the hyperbolic sine.
      sinh }
    { /// Computes the hyperbolic cosine.
     cosh }
    { /// Computes the hyperbolic tangent.
      tanh }
    { /// Computes the inverse hyperbolic sine.
      asinh }
    { /// Computes the inverse hyperbolic cosine.
      acosh }
    { /// Computes the inverse hyperbolic tangent.
      atanh }
);
*/

// TODO: Add this to te python API once this is done - https://github.com/PyO3/pyo3/issues/1190
impl num_traits::Zero for StandardForm {
    fn zero() -> Self {
        Self::new_unchecked(Finite::zero(), num_traits::Zero::zero())
    }

    fn is_zero(&self) -> bool {
        self.mantissa.is_zero() && self.exponent.is_zero()
    }
}

impl num_traits::One for StandardForm {
    fn one() -> Self {
        Self::new_unchecked(Finite::one(), num_traits::Zero::zero())
    }
}

impl num_traits::Num for StandardForm {
    type FromStrRadixErr = crate::ParsingStandardFormError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match radix != 10 {
            true => Err(crate::ParsingStandardFormError::InvalidRadix),
            false => Self::try_from(s),
        }
    }
}

impl num_traits::Signed for StandardForm {
    #[must_use]
    fn abs(&self) -> Self {
        Self::new_unchecked(self.mantissa().abs(), *self.exponent())
    }

    #[must_use]
    fn abs_sub(&self, other: &Self) -> Self {
        match *self <= *other {
            true => Self::zero(),
            false => self.clone() - other.clone(),
        }
    }

    #[must_use]
    fn signum(&self) -> Self {
        match self.mantissa().into_inner().signum() as i8 {
            1 => Self::one(),
            0 => Self::zero(),
            _ => -Self::one(),
        }
    }

    #[must_use]
    fn is_positive(&self) -> bool {
        self.mantissa().is_sign_positive()
    }

    #[must_use]
    fn is_negative(&self) -> bool {
        self.mantissa().is_sign_negative()
    }
}

impl num_traits::FromPrimitive for StandardForm {
    // Required methods
    #[must_use]
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self::from(n))
    }
    #[must_use]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::from(n))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Self::try_from(n).ok()
    }

    fn from_f32(n: f32) -> Option<Self> {
        Self::try_from(n).ok()
    }
}

impl num_traits::ToPrimitive for StandardForm {
    fn to_f64(&self) -> Option<f64> {
        10i8.checked_pow(self.exponent as u32)
            .map(|v: i8| self.mantissa.into_inner() * (v as f64))
    }

    fn to_i64(&self) -> Option<i64> {
        self.to_f64().and_then(|val| {
            // Check for special cases (NaN, Inf, etc.)
            if val.is_nan() || val.is_infinite() {
                return None;
            }

            // Check if the value is within the valid range for i64
            if val < i64::MIN as f64 || val > i64::MAX as f64 {
                return None;
            }

            // Convert safely
            Some(val as i64)
        })
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_i64().and_then(|val| val.try_into().ok())
    }
}

const FIRST_SMALLER_VALUE_THEN_10: f64 = 10.0 - f64::EPSILON;
impl num_traits::Bounded for StandardForm {
    fn max_value() -> Self {
        Self::new_unchecked(FIRST_SMALLER_VALUE_THEN_10.into(), i8::MAX)
    }

    fn min_value() -> Self {
        Self::new_unchecked((-FIRST_SMALLER_VALUE_THEN_10).into(), i8::MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const FALIURE_MSG: &str = "Failed parsing standardform";

    #[test_case( 1.0, 2, "1.00e+2"; "already_in_scientific_notation")]
    #[test_case( 0.01, 4, "1.00e+2"; "small_number_adjustment")]
    #[test_case( 1234.0, 0, "1.234e+3"; "large_number_adjustment")]
    #[test_case( -5678.9, 2, "-5.6789e+5"; "negative_number_adjustment")]
    #[test_case( 0.000567, -3, "5.67e-7"; "small_negative_exponent_adjustment")]
    #[test_case( 98765.432, 3, "9.8765432e+7"; "large_mantissa_adjustment")]
    #[test_case( 0.99999, -1, "9.9999e-2"; "rounding_edge_case")]
    #[test_case( 1.5, 10, "1.50e+10"; "zero_adjustment_required")]
    #[test_case( -0.0001, 2, "-1.00e-2"; "negative_small_number_adjustment")]
    #[test_case( 999.999, -5, "9.99999e-3"; "high_precision_adjustment")]
    fn correct_adjustment(mantissa: f64, exponent: i8, expected_in_science_notation: &str) {
        assert_eq!(
            StandardForm::new(Finite::from_inner(mantissa), exponent).to_scientific_notation(),
            StandardForm::try_from(expected_in_science_notation)
                .expect(FALIURE_MSG)
                .to_scientific_notation()
        )
    }

    #[test_case("1.0e2", "1.00e+2"; "parse_correct_scientific_notation")]
    #[test_case("12345.678", "1.2345678e+4"; "parse_large_number")]
    #[test_case("0.000567", "5.67e-4"; "parse_small_number")]
    #[test_case("1.2345e3", "1.2345e+3"; "parse_exponent_included")]
    #[test_case("-5678.9", "-5.6789e+3"; "parse_negative_number")]
    #[test_case("0.00000123", "1.23e-6"; "parse_tiny_number")]
    #[test_case("9999999.0", "9.999999e6"; "parse_large_integer")]
    #[test_case("-0.005", "-5.00e-3"; "parse_negative_small_number")]
    #[test_case("98765e1", "9.8765e+5"; "parse_large_scientific")]
    #[test_case("0.0012345678", "1.2345678e-3"; "parse_high_precision")]
    fn parsing_standardform(input: &str, expected_in_science_notation: &str) {
        assert_eq!(
            StandardForm::try_from(input).map(|v| v.to_scientific_notation()),
            Ok(StandardForm::try_from(expected_in_science_notation)
                .expect(FALIURE_MSG)
                .to_scientific_notation())
        )
    }

    #[test_case("1.0e2", "2.0e2", "3.00e+2"; "addition_simple")]
    #[test_case("1.5e3", "2.5e3", "4.00e+3"; "addition_large_numbers")]
    #[test_case("1.0e-2", "2.0e-2", "3.00e-2"; "addition_small_numbers")]
    #[test_case("-1.0e3", "2.0e3", "1.00e+3"; "addition_with_negative")]
    #[test_case("3.5e4", "4.5e4", "8.00e+4"; "addition_similar_exponent")]
    #[test_case("5.0e1", "5.0e2", "5.50e+2"; "addition_different_exponent")]
    #[test_case("1.23e2", "-2.46e2", "-1.23e+2"; "addition_negative_result")]
    #[test_case("1.0e6", "1.0e6", "2.00e+6"; "addition_large_equal_numbers")]
    #[test_case("5.0e-5", "5.0e-5", "1.00e-4"; "addition_small_equal_numbers")]
    #[test_case("9.9e1", "1.1e1", "1.10e+2"; "addition_with_carry")]
    fn add_standardform(lhs: &str, rhs: &str, expected_in_science_notation: &str) {
        let lhs = StandardForm::try_from(lhs).expect(FALIURE_MSG);
        let rhs = StandardForm::try_from(rhs).expect(FALIURE_MSG);

        assert_eq!(
            (lhs + rhs).to_scientific_notation(),
            StandardForm::try_from(expected_in_science_notation)
                .expect(FALIURE_MSG)
                .to_scientific_notation()
        )
    }

    #[test_case("3.0e2", "2.0e2", "1.00e+2"; "subtraction_simple")]
    #[test_case("5.0e3", "2.0e3", "3.00e+3"; "subtraction_large_numbers")]
    #[test_case("2.0e-2", "1.0e-2", "1.00e-2"; "subtraction_small_numbers")]
    #[test_case("1.0e6", "1.0e5", "9.00e+5"; "subtraction_large_difference")]
    #[test_case("7.0e2", "3.0e2", "4.00e+2"; "subtraction_with_carry")]
    #[test_case("1.23e2", "-2.46e2", "3.69e+2"; "subtraction_negative_term")]
    #[test_case("5.0e-3", "1.0e-3", "4.00e-3"; "subtraction_small_difference")]
    #[test_case("1.0e2", "9.0e1", "1.00e+1"; "subtraction_near_equal")]
    #[test_case("-1.0e3", "-1.0e2", "-9.00e+2"; "subtraction_negative_numbers")]
    #[test_case("1.0e1", "5.0e0", "5.00e+0"; "subtraction_half_value")]
    fn sub_standardform(lhs: &str, rhs: &str, expected_in_science_notation: &str) {
        let lhs = StandardForm::try_from(lhs).expect(FALIURE_MSG);
        let rhs = StandardForm::try_from(rhs).expect(FALIURE_MSG);

        assert_eq!(
            (lhs - rhs).to_scientific_notation(),
            StandardForm::try_from(expected_in_science_notation)
                .expect(FALIURE_MSG)
                .to_scientific_notation()
        )
    }

    #[test_case("1.0e2", "2.0e2", "5.00e-1"; "division_simple")]
    #[test_case("5.0e3", "2.0e3", "2.50e+0"; "division_large_numbers")]
    #[test_case("2.0e-2", "1.0e-2", "2.00e+0"; "division_small_numbers")]
    #[test_case("1.0e6", "1.0e3", "1.00e+3"; "division_large_by_large")]
    #[test_case("9.0e2", "3.0e2", "3.00e+0"; "division_exact_division")]
    #[test_case("7.0e5", "1.0e3", "7.00e+2"; "division_no_rounding")]
    #[test_case("8.0e-4", "2.0e-2", "4.00e-2"; "division_small_by_small")]
    #[test_case("5.0e2", "5.0e1", "1.00e+1"; "division_by_tenth")]
    #[test_case("1.0e1", "2.0e1", "5.00e-1"; "division_half")]
    #[test_case("1.0e-1", "5.0e-1", "2.00e-1"; "division_small_numbers_with_remainder")]
    fn div_standardform(lhs: &str, rhs: &str, expected_in_science_notation: &str) {
        let lhs = StandardForm::try_from(lhs).expect(FALIURE_MSG);
        let rhs = StandardForm::try_from(rhs).expect(FALIURE_MSG);

        assert_eq!(
            (lhs / rhs).to_scientific_notation(),
            StandardForm::try_from(expected_in_science_notation)
                .expect(FALIURE_MSG)
                .to_scientific_notation()
        )
    }

    #[test_case("1.0e2", "2.0e2", "2.00e+4"; "multiplication_simple")]
    #[test_case("5.0e3", "2.0e3", "1.00e+7"; "multiplication_large_numbers")]
    #[test_case("2.0e-2", "1.0e-2", "2.00e-4"; "multiplication_small_numbers")]
    #[test_case("1.0e6", "1.0e-3", "1.00e+3"; "multiplication_large_by_small")]
    #[test_case("9.0e2", "3.0e2", "2.70e+5"; "multiplication_no_rounding")]
    #[test_case("7.0e5", "1.0e3", "7.00e+8"; "multiplication_large_by_large")]
    #[test_case("8.0e-4", "2.0e-2", "1.60e-5"; "multiplication_small_by_small")]
    #[test_case("5.0e2", "5.0e1", "2.50e+4"; "multiplication_by_tenth")]
    #[test_case("1.0e1", "2.0e1", "2.00e+2"; "multiplication_basic")]
    #[test_case("1.0e-1", "5.0e-1", "5.00e-2"; "multiplication_small_numbers2")]
    fn mul_standardform(lhs: &str, rhs: &str, expected_in_science_notation: &str) {
        let lhs = StandardForm::try_from(lhs).expect(FALIURE_MSG);
        let rhs = StandardForm::try_from(rhs).expect(FALIURE_MSG);

        assert_eq!(
            (lhs * rhs).to_scientific_notation(),
            StandardForm::try_from(expected_in_science_notation)
                .expect(FALIURE_MSG)
                .to_scientific_notation()
        )
    }

    // TODO - Ensure these tests pass
    #[test_case("5.0e2", "1.0e2", "0.00e+0"; "remainder_simple_whole")]
    #[test_case("5.5e2", "1.0e2", "5.00e+1"; "remainder_simple_fraction")]
    #[test_case("1.0e5", "3.0e4", "1.00e+4"; "remainder_large_numbers")]
    #[test_case("1.0e2", "3.0e1", "1.00e+1"; "remainder_large_by_smaller")]
    #[test_case("7.0e5", "4.0e5", "3.00e+5"; "remainder_no_rounding")]
    #[test_case("1.2e-1", "5.0e-2", "2.00e-2"; "remainder_small_numbers")]
    #[test_case("9.9e1", "2.0e1", "1.90e+1"; "remainder_with_fraction")]
    #[test_case("5.0e3", "1.5e3", "5.00e+2"; "remainder_fraction_result")]
    #[test_case("4.5e4", "1.0e4", "5.00e+3"; "remainder_exact_half")]
    #[test_case("2.3e-1", "1.1e-1", "1.00e-2"; "remainder_small_fraction")]
    fn rem_standardform(lhs: &str, rhs: &str, expected_in_science_notation: &str) {
        let lhs = StandardForm::try_from(lhs).expect(FALIURE_MSG);
        let rhs = StandardForm::try_from(rhs).expect(FALIURE_MSG);

        assert_eq!(
            (lhs % rhs).to_scientific_notation(),
            StandardForm::try_from(expected_in_science_notation)
                .expect(FALIURE_MSG)
                .to_scientific_notation()
        )
    }
}
