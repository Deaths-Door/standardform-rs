use num_traits::{
    identities::{One,Zero},
    sign::Signed,
    cast::FromPrimitive,
    ToPrimitive,
    Num,
    Pow
};

use crate::{StandardForm,ParsingStandardFormError};

/// Represents a standard form number with zero mantissa and zero exponent.
pub static ZERO: StandardForm = StandardForm::new_unchecked(0.0, 0);

/// Represents a standard form number with a mantissa of 1.0 and zero exponent.
pub static ONE: StandardForm = StandardForm::new_unchecked(1.0, 0);

impl Zero for StandardForm {
    
    #[must_use]
    fn zero() -> StandardForm {
        StandardForm::new_unchecked(0.0,0)
    }
    #[must_use]
    fn is_zero(&self) -> bool {
        self.mantissa() == &0.0
    }
}

impl One for StandardForm {
    #[must_use]
    fn one() -> StandardForm {
        StandardForm::new_unchecked(1.0,0)
    }
}

impl Num for StandardForm {
    type FromStrRadixErr = ParsingStandardFormError;
    #[inline]
    #[must_use]
    fn from_str_radix(s: &str, radix: u32)-> Result<Self,Self::FromStrRadixErr> {
        match radix != 10 {
            true => Err(ParsingStandardFormError::InvalidRadix),
            false => Self::try_from(s)
        }
    }
}

impl Signed for StandardForm {
    #[must_use]
    fn abs(&self) -> Self {
        Self::new_unchecked(self.mantissa().abs(),*self.exponent())
    }

    #[must_use]
    fn abs_sub(&self, other: &Self) -> Self {
        match *self <= *other {
            true => Self::zero(),
            false => self.clone() - other.clone()
        }
    }
    
    #[must_use]
    fn signum(&self) -> Self {
        match self.mantissa().signum() as i8 {
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

impl FromPrimitive for StandardForm {
    // Required methods
    #[must_use]
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self::new(n as f64,0))
    }
    #[must_use]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::new(n as f64,0))
    }
}

impl ToPrimitive for StandardForm {
    // Required methods
    #[must_use]
    fn to_i64(&self) -> Option<i64> {
        let x : f64 = self.clone().into();
        Some(x as i64)
    }
    
    #[must_use]
    fn to_u64(&self) -> Option<u64> {
        let x : f64 = self.clone().into();
        Some(x as u64)
    }
}

impl Pow<Self> for StandardForm {
    type Output = f64;

    #[must_use]
    fn pow(self, other: Self) -> Self::Output {
        let x : f64 = self.into();
        let y : f64 = other.into();
        x.powf(y)
    }
}