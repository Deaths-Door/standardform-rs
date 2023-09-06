use num_traits::{
    identities::{One,Zero},
    sign::Signed,
    cast::FromPrimitive,
    Num
};

use crate::{StandardForm,ParsingStandardFormError};

impl Zero for StandardForm {
    fn zero() -> StandardForm {
        StandardForm::new_unchecked(0.0,0)
    }

    fn is_zero(&self) -> bool {
        self.mantissa() == &0.0
    }
}

impl One for StandardForm {
    fn one() -> StandardForm {
        StandardForm::new_unchecked(1.0,0)
    }
}

impl Num for StandardForm {
    type FromStrRadixErr = ParsingStandardFormError;
    #[inline]
    fn from_str_radix(s: &str, radix: u32)-> Result<Self,Self::FromStrRadixErr> {
        match radix != 10 {
            true => Err(ParsingStandardFormError::InvalidRadix),
            false => Self::try_from(s)
        }
    }
}

impl Signed for StandardForm {
    fn abs(&self) -> Self {
        Self::new_unchecked(self.mantissa().abs(),*self.exponent())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        match *self <= *other {
            true => Self::zero(),
            false => self.clone() - other.clone()
        }
    }
    
    fn signum(&self) -> Self {
        match self.mantissa().signum() as i8 {
            1 => Self::one(),
            0 => Self::zero(),
            _ => -Self::one(),
        }
    }

    fn is_positive(&self) -> bool {
        self.mantissa().is_sign_positive()
    }

    fn is_negative(&self) -> bool {
        self.mantissa().is_sign_negative()
    }
}

impl FromPrimitive for StandardForm {
    // Required methods
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self::new(n as f64,0))
    }
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::new(n as f64,0))
    }
}