use num_traits::{
    identities::{One,Zero},
    sign::Signed,
    cast::FromPrimitive,
    Num,
    Pow
};

use crate::{StandardForm,ParsingStandardFormError,round};

/// Represents a standard form number with zero mantissa and zero exponent.
pub static ZERO: StandardForm = StandardForm::new_unchecked(0.0, 0);

/// Represents a standard form number with a mantissa of 1.0 and zero exponent.
pub static ONE: StandardForm = StandardForm::new_unchecked(1.0, 0);

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

impl Pow<Self> for StandardForm {
    type Output = Self;
    fn pow(self, other: Self) -> Self::Output {
        let new_mantissa = self.mantissa().powf(*other.mantissa());
        let new_exponent = *self.exponent() as f64 * other.mantissa() as i8;

        StandardForm::new(new_mantissa, new_exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
fn test_standard_form_pow() {
    let sf = StandardForm::new(2.0, 3);
    
    // 2^3 = 8
    let result = sf.pow(3);
    assert_eq!(result, StandardForm::new(8.0, 3));

    // 2^0 = 1
    let result = sf.pow(0);
    assert_eq!(result, StandardForm::new(1.0, 0));

    // 2^(-2) = 1/4 = 0.25
    let result = sf.pow(-2);
    assert_eq!(result, StandardForm::new(0.25, -2));
}


    #[test]
    fn test_pow_positive_exponent() {
        // Test positive exponent
        let base = StandardForm::new_unchecked(2.0, 3); // 20
        let exponent = StandardForm::new_unchecked(3.0, 2); // 3
        
        let result = base.pow(exponent);
        
        let expected = StandardForm::from(8000_i32);
        
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_pow_negative_exponent() {
        // Test negative exponent
        let base = StandardForm::new_unchecked(5.0, -2); // 5.0e-2
        let exponent = StandardForm::new_unchecked(2.0, 4); // 2.0e4
        
        let result = base.pow(exponent);
        
        let expected_mantissa = 25.0; // 5.0^2.0
        let expected_exponent = 2; // (-2) * 4
        let expected = StandardForm::new_unchecked(expected_mantissa, expected_exponent);
        
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_pow_zero_exponent() {
        // Test zero exponent
        let base = StandardForm::new_unchecked(7.0, 2); // 7.0e2
        let exponent = StandardForm::new_unchecked(1.0, 0); // 1.0e0
        
        let result = base.pow(exponent);
        
        let expected_mantissa = 1.0; // 7.0^1.0
        let expected_exponent = 2; // 2
        let expected = StandardForm::new_unchecked(expected_mantissa, expected_exponent);
        
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_pow_one_base() {
        // Test base equal to 1.0
        let base = StandardForm::new_unchecked(1.0, 4); // 1.0e4
        let exponent = StandardForm::new_unchecked(3.0, -2); // 3.0e-2
        
        let result = base.pow(exponent);
        
        let expected_mantissa = 1.0; // 1.0^3.0
        let expected_exponent = 4; // 4 + (-2)
        let expected = StandardForm::new_unchecked(expected_mantissa, expected_exponent);
        
        assert_eq!(result, expected);
    }
}