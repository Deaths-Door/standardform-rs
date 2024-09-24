#![doc = include_str!("../README.md")]
#![forbid(
    missing_docs,
    missing_debug_implementations,
    unsafe_code,
    unused_mut,
    unused_allocation,
    unused_must_use,
    unreachable_patterns,
    unstable_features,
    unsafe_op_in_unsafe_fn,
    trivial_casts,
    overflowing_literals,
    non_snake_case
)]

#[forbid(
    clippy::cargo,
    clippy::perf,
    clippy::complexity,
    clippy::style,
    clippy::suspicious
)]
use decorum::Finite;
use num_traits::real::Real;
use num_traits::{One, Zero};
use std::i8;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;

use thiserror::Error;

/// Represents a number in standard form.
///
/// The `Standardform` struct holds the significand (mantissa) of the number
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
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
    #[must_use]
    pub fn new(mantissa: Finite<f64>, exponent: i8) -> Self {
        let mut instance = Self::new_unchecked(mantissa, exponent);
        instance.adjust();
        instance
    }

    pub(crate) const fn new_unchecked(mantissa: Finite<f64>, exponent: i8) -> Self {
        Self { mantissa, exponent }
    }

    fn in_range(&self) -> bool {
        (self.mantissa >= 1.0 && self.mantissa <= 10.0)
            || (self.mantissa >= -10.0 && self.mantissa <= -1.0)
    }

    fn adjust(&mut self) {
        if self.in_range() || self.mantissa == 0.0 {
            return;
        }

        use num_traits::real::Real;

        let log10_mantissa = self.mantissa.abs().log10();
        let adjustment = log10_mantissa.floor(); // Find the largest power of 10

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

        write!(
            f,
            "{}",
            self.mantissa * 10_f64.powi(self.exponent as i32) as f64
        )
    }
}

impl StandardForm {
    /// Returns a reference to the StandardForm representing the significand (mantissa) of the number.
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn mantissa(&self) -> &Finite<f64> {
        &self.mantissa
    }

    /// Returns the exponent that determines the power of 10 by which the significand should be multiplied.
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn exponent(&self) -> &i8 {
        &self.exponent
    }
}

impl StandardForm {
    /// Returns the string representation of the number in scientific notation.
    #[must_use]
    #[cfg(feature = "std")]
    pub fn to_scientific_notation(&self) -> String {
        format!("{}e{}", self.mantissa, self.exponent)
    }

    /// Returns the string representation of the number in engineering notation.
    #[must_use]
    #[cfg(feature = "std")]
    pub fn to_engineering_notation(&self) -> String {
        format!("{}*10^{}", self.mantissa, self.exponent)
    }
}

#[cfg(feature = "std")]
impl TryFrom<&str> for StandardForm {
    type Error = crate::ParsingStandardFormError;

    #[must_use]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(number) = value.parse::<f64>() {
            let output: Result<Self, ConversionError> = number.try_into();
            return output.map_err(|e| e.into());
        }

        if let Some(index) = value.find('e') {
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
#[derive(Error, Debug, Clone)]
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

    ///
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

        let x = self.mantissa * 10_f64.powi((self.exponent - min) as i32) as f64;
        let y = other.mantissa * 10_f64.powi((other.exponent - min) as i32) as f64;

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

impl core::ops::RemAssign for StandardForm {
    fn rem_assign(&mut self, other: StandardForm) {
        *self = self.clone() % other;
    }
}

impl core::ops::Rem for StandardForm {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let output = self / rhs;
        let fract = output.mantissa.fract();

        let mantissa = (fract * output.exponent as f64).fract();

        StandardForm::new(mantissa, Zero::zero())
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

primitives!(ints => u8,u16,u32
    ,u64,i8,i16,i32,i64);
primitives!(ops => Finite<f64>);

impl From<Finite<f64>> for StandardForm {
    fn from(value: Finite<f64>) -> Self {
        Self::new(value, Zero::zero())
    }
}

#[derive(Debug, Clone, thiserror::Error)]
///
pub enum ConversionError {
    ///
    #[error("Constraint of Finite<f64> is voliated by NaN")]
    NaN,

    ///
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

impl num_traits::Bounded for StandardForm {
    fn max_value() -> Self {
        Self::new_unchecked(9.9999999999999999.into(), i8::MAX)
    }

    fn min_value() -> Self {
        Self::new_unchecked((-9.9999999999999999).into(), i8::MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // Helper function to assert equality of StandardForm numbers
    fn assert_standardform_eq(sf: &StandardForm, expected_mantissa: f64, expected_exponent: i8) {
        assert_eq!(sf.mantissa.into_inner(), expected_mantissa);
        assert_eq!(sf.exponent, expected_exponent);
    }

    // Test cases for StandardForm::new
    #[test_case(1.0, 3, 1.0, 3; "normal case")]
    #[test_case(0.5, -5, 5.0, -6; "small mantissa, negative exponent")]
    #[test_case(9.999999999999999, 100, 9.999999999999999, 100; "large mantissa, positive exponent")]
    #[test_case(-2.5, 0, -2.5, 0; "negative mantissa, zero exponent")]
    fn test_standardform_new(
        mantissa: f64,
        exponent: i8,
        expected_mantissa: f64,
        expected_exponent: i8,
    ) {
        let sf = StandardForm::new(mantissa.into(), exponent);
        assert_standardform_eq(&sf, expected_mantissa, expected_exponent);
    }

    // Test cases for addition
    #[test_case(1.0, 2, 2.0, 2, 3.0, 2; "simple addition with same exponent")]
    #[test_case(1.0, 1, 1.0, 1, 2.0, 1; "simple addition with exponent 1")]
    #[test_case(1.0, 2, 1.0, 1, 1.1, 2; "adding values with different exponents")]
    fn test_standardform_add(
        m1: f64,
        e1: i8,
        m2: f64,
        e2: i8,
        expected_mantissa: f64,
        expected_exponent: i8,
    ) {
        let sf1 = StandardForm::new(m1.into(), e1);
        let sf2 = StandardForm::new(m2.into(), e2);
        let result = sf1 + sf2;
        assert_standardform_eq(&result, expected_mantissa, expected_exponent);
    }

    // Test cases for subtraction
    #[test_case(3.0, 2, 1.0, 2, 2.0, 2; "simple subtraction with same exponent")]
    #[test_case(1.0, 1, 0.5, 1, 1.5, 1; "simple subtraction with exponent 1")]
    #[test_case(1.0, 2, 0.1, 1, 1.01, 2; "subtracting values with different exponents")]
    fn test_standardform_sub(
        m1: f64,
        e1: i8,
        m2: f64,
        e2: i8,
        expected_mantissa: f64,
        expected_exponent: i8,
    ) {
        let sf1 = StandardForm::new(m1.into(), e1);
        let sf2 = StandardForm::new(m2.into(), e2);
        let result = sf1 - sf2;
        assert_standardform_eq(&result, expected_mantissa, expected_exponent);
    }

    // Test cases for multiplication
    #[test_case(2.0, 1, 3.0, 1, 6.0, 2; "simple multiplication with same exponent")]
    #[test_case(1.5, 1, 2.0, 0, 3.0, 1; "multiplication with different exponents")]
    fn test_standardform_mul(
        m1: f64,
        e1: i8,
        m2: f64,
        e2: i8,
        expected_mantissa: f64,
        expected_exponent: i8,
    ) {
        let sf1 = StandardForm::new(m1.into(), e1);
        let sf2 = StandardForm::new(m2.into(), e2);
        let result = sf1 * sf2;
        assert_standardform_eq(&result, expected_mantissa, expected_exponent);
    }

    // Test cases for division
    #[test_case(6.0, 2, 3.0, 1, 2.0, 1; "simple division with same exponent")]
    #[test_case(4.0, 2, 2.0, 0, 2.0, 2; "division with different exponents")]
    fn test_standardform_div(
        m1: f64,
        e1: i8,
        m2: f64,
        e2: i8,
        expected_mantissa: f64,
        expected_exponent: i8,
    ) {
        let sf1 = StandardForm::new(m1.into(), e1);
        let sf2 = StandardForm::new(m2.into(), e2);
        let result = sf1 / sf2;
        assert_standardform_eq(&result, expected_mantissa, expected_exponent);
    }

    // Test cases for remainder (modulus)
    #[test_case(7.0, 1, 3.0, 1, 1.0, 1; "simple modulus with same exponent")]
    #[test_case(10.0, 2, 3.0, 1, 1.0, 2; "modulus with different exponents")]
    fn test_standardform_rem(
        m1: f64,
        e1: i8,
        m2: f64,
        e2: i8,
        expected_mantissa: f64,
        expected_exponent: i8,
    ) {
        let sf1 = StandardForm::new(m1.into(), e1);
        let sf2 = StandardForm::new(m2.into(), e2);
        let result = sf1 % sf2;
        assert_standardform_eq(&result, expected_mantissa, expected_exponent);
    }
}
