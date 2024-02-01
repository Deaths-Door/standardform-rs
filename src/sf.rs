use core::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign,Neg,Rem,RemAssign};
use core::cmp::Ordering;

#[cfg(feature="js")]
use wasm_bindgen::prelude::*;

#[cfg(feature="bindings")]
use uniffi::*;

/// Represents a number in standard form.
///
/// The `Standardform` struct holds the significand (mantissa) of the number 
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
#[derive(Clone,PartialEq)]
#[cfg_attr(feature="js", wasm_bindgen)]
#[cfg_attr(feature="bindings", derive(Object))]
pub struct StandardForm  {
    mantissa : f64,
    exponent : i8 
}

#[cfg_attr(feature="js", wasm_bindgen)]
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
    #[cfg_attr(feature="js", wasm_bindgen(constructor))]
    #[cfg_attr(feature="bindings", constructor)]
    pub fn new(mantissa : f64,exponent : i8) -> Self {
        let mut instance = Self::new_unchecked(mantissa,exponent);
        instance.adjust();
        instance
    }

   // #[cfg_attr(all(feature="js",feature="std"), wasm_bindgen)]
    #[cfg(feature="js")]
    #[wasm_bindgen]
    /// Converts string into `StandardFrom` as traits cannot be 'bridged' 
    pub fn new_from_string(string : &str) -> Result<StandardForm,JsValue> {
        Self::try_from(string).map_err(|e| JsValue::from_str(&e.to_string()))
    }


    #[cfg_attr(feature="js", wasm_bindgen)]
    /// Converts `StandardFrom` into f64 as traits cannot be 'bridged' 
    pub fn into_f64(self) -> f64 {
        self.into()
    }

    pub(crate) const fn new_unchecked(mantissa : f64,exponent : i8) -> Self { 
        Self { mantissa , exponent }
    }

    fn in_range(&self) -> bool {
        (self.mantissa >= 1.0 && self.mantissa <= 10.0) || (self.mantissa >= -10.0 && self.mantissa <= -1.0)
    }

    fn adjust(&mut self) {
        if self.in_range() || self.mantissa == 0.0 {
            return;
        }

        // means its things like 0.2323 not -700
        match self.mantissa > -1.0 && self.mantissa < 1.0 {
            true => while !self.in_range() {
                self.mantissa *= 10.0;
                self.exponent -= 1; 
            },
            false => while !self.in_range() {
                self.mantissa /= 10.0;
                self.exponent += 1; 
            }
        }
    }
}

#[cfg_attr(feature="bindings",export)]
impl StandardForm {
    /// Returns a reference to the StandardForm representing the significand (mantissa) of the number.
    #[cfg(not(feature="js"))]
    #[must_use]
    pub const fn mantissa(&self) -> &f64 {
        &self.mantissa
    }

    /// Returns a reference to the StandardForm representing the significand (mantissa) of the number.
    #[must_use]
    #[cfg(feature="js")]
    pub fn mantissa(&self) -> &f64 {
        &self.mantissa
    }
    
    /// Returns the exponent that determines the power of 10 by which the significand should be multiplied.
    #[cfg(not(feature="js"))]
    #[must_use]
    pub const fn exponent(&self) -> &i8 {
        &self.exponent
    }  

    /// Returns the exponent that determines the power of 10 by which the significand should be multiplied.
    #[must_use]
    #[cfg(feature="js")]
    pub const fn exponent(&self) -> &i8 {
        &self.exponent
    }      
}

#[cfg_attr(feature="js", wasm_bindgen)]
impl StandardForm {
    /// Returns the string representation of the number in scientific notation.
    #[must_use]
    #[cfg_attr(feature="js", wasm_bindgen)]
    pub fn to_scientific_notation(&self) -> String {
        format!("{}e{}", self.mantissa, self.exponent)
    }
        
    /// Returns the string representation of the number in engineering notation.
    #[must_use]
    #[cfg_attr(feature="js", wasm_bindgen)]
    pub fn to_engineering_notation(&self) -> String {
        format!("{}*10^{}", self.mantissa, self.exponent)
    }    
}

impl Default for StandardForm {
    #[must_use]
    fn default() -> Self {
        Self { mantissa : 1.0, exponent : 0 }
    }
}

impl PartialOrd for StandardForm {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.exponent == other.exponent {
            true => self.mantissa.partial_cmp(&other.mantissa),
            false => self.exponent.partial_cmp(&other.exponent)
        }
    }
}

impl Eq for StandardForm {}

impl Ord for StandardForm {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl core::fmt::Display for StandardForm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.exponent > 4 {
            return write!(f,"{}",self.to_scientific_notation());
        };

        write!(f,"{}",self.mantissa * 10_f64.powi(self.exponent as i32) as f64)
    }
}

impl core::fmt::Debug for StandardForm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f,"{self}")
    }
}

impl From<StandardForm> for f64 {
    #[must_use]
    fn from(value: StandardForm) -> Self {
       value.mantissa * 10_f64.powi(value.exponent as i32)
    }
}

#[cfg(feature="std")]
impl TryFrom<&str> for StandardForm {
    type Error = crate::ParsingStandardFormError;

    #[must_use]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(number) = value.parse::<f64>() {
            return Ok(number.into());
        }

        if let Some(index) = value.find('e') {
            let m_str : f64 = value[0..index].parse()?;
            let e_str : i8 = value[index + 1..].parse()?;
            return Ok(StandardForm::new(m_str,e_str));
        }
        
        if let Some(index) = value.find('^') {
            let m_str : f64 = value[0..index - 3].parse()?;
            let e_str : i8 = value[index + 1..].parse()?;
            return Ok(StandardForm::new(m_str,e_str));
        }

        Err(crate::ParsingStandardFormError::InvalidFormat)
    }
}

#[cfg(feature="std")]
impl TryFrom<&[u8]> for StandardForm {
    type Error = crate::ParsingStandardFormError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(std::str::from_utf8(value)?)
    }
}

impl Neg for StandardForm {
    type Output = Self;
    #[must_use]
    fn neg(self) -> Self::Output {
        Self::new_unchecked(-self.mantissa,self.exponent)
    }
}

impl Rem for StandardForm {
    type Output = Self;
    #[must_use]
    fn rem(self,other : Self) -> Self::Output {
        self.clone() - (self / other.clone() * other) 
    }
}

impl Add for StandardForm {
    type Output = Self;
    #[must_use]
    fn add(self, other: Self) -> Self {
        let max_power = self.exponent.max(other.exponent);
        let num_sum = self.mantissa * 10.0_f64.powf((self.exponent - max_power) as f64) + other.mantissa * 10.0_f64.powf((other.exponent - max_power) as f64);
        StandardForm::new(num_sum, max_power)
    }
}

impl AddAssign for StandardForm {
    fn add_assign(&mut self, other: Self) {
        let max_power = self.exponent.max(other.exponent);
        let num_sum = self.mantissa * 10.0_f64.powf((self.exponent - max_power) as f64) + other.mantissa * 10.0_f64.powf((other.exponent - max_power) as f64);

        self.mantissa = num_sum;
        self.exponent = max_power;

        self.adjust();
    }
}

const TOLERANCE : f64 = 1.0e6;

pub(crate) fn round(result : f64) -> f64 {
    (result * TOLERANCE).round() / TOLERANCE
}

impl Sub for StandardForm {
    type Output = Self;
    #[must_use]
    fn sub(self, other: Self) -> Self {
        let min = self.exponent.min(other.exponent);

        let x = self.mantissa * 10_f64.powi((self.exponent - min) as i32) as f64;
        let y = other.mantissa * 10_f64.powi((other.exponent - min) as i32) as f64;

        let result = x - y;

        StandardForm::new(round(result),min)
    }
}

impl SubAssign for StandardForm {
    fn sub_assign(&mut self, other: Self) {
        let min = self.exponent.min(other.exponent);

        let x = self.mantissa * 10_i32.pow((self.exponent - min) as u32) as f64;
        let y = other.mantissa * 10_i32.pow((other.exponent - min) as u32) as f64;

        let result = x - y;

        self.mantissa = round(result);
        self.exponent = min;
        self.adjust(); 
    }
}

impl Mul for StandardForm {
    type Output = Self;
    #[must_use]
    fn mul(self, other: Self) -> Self {
        let exponent = self.exponent + other.exponent;
        let mantissa = self.mantissa * other.mantissa;
        StandardForm::new(round(mantissa),exponent)
    }
}

impl MulAssign for StandardForm {
    fn mul_assign(&mut self, other: Self) {
        let exponent = self.exponent + other.exponent;
        let mantissa = self.mantissa * other.mantissa;

        self.mantissa = round(mantissa);
        self.exponent = exponent;

        self.adjust();
    }
}

impl Div for StandardForm {
    type Output = Self;
    #[must_use]
    fn div(self, other: Self) -> Self {
        StandardForm::new(self.mantissa / other.mantissa,self.exponent - other.exponent)
    }
}

impl DivAssign for StandardForm {
    fn div_assign(&mut self, other: Self) {
        self.mantissa /= other.mantissa;
        self.exponent /= other.exponent;
        self.adjust();
    }
}

impl RemAssign for StandardForm {
    fn rem_assign(&mut self, other: StandardForm) {
        *self = self.clone() % other;
    }
}

macro_rules! primitives {
    (form => $($t:ty),*) => {
        $(
            impl From<$t> for StandardForm {
                #[must_use]
                fn from(value: $t) -> Self {
                    StandardForm::new(value as f64,0)
                }
            }
        )*
    };

    (eq => $($t:ty),*) => {
        $(
            impl PartialEq<$t> for StandardForm {
                fn eq(&self,other: &$t) -> bool {
                    let rhs : Self = (*other).into();
                    *self == rhs
                }
            }
        )*
    };
    (ord => $($t:ty),*) => {
        $(
            impl PartialOrd<$t> for StandardForm {
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    let rhs : Self = (*other).into();
                    self.partial_cmp(&rhs)
                }
            }
        )*
    };


    (add => $($t : ty),*) => {
        $(
            impl Add<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn add(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self + rhs
                }
            }
            
            impl AddAssign<$t> for StandardForm {
                fn add_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self += rhs;
                }
            }
        )*
    };

    (sub => $($t : ty),*) => {
        $(
            impl Sub<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn sub(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self - rhs
                }
            }
            
            impl SubAssign<$t> for StandardForm {
                fn sub_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self -= rhs;
                }
            }
        )*
    };
    (mul => $($t : ty),*) => {
        $(
            impl Mul<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn mul(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self * rhs
                }
            }
            
            impl MulAssign<$t> for StandardForm {
                fn mul_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self *= rhs;
                }
            }
        )*
    };
    (div => $($t : ty),*) => {
        $(
            impl Div<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn div(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self / rhs
                }
            }
            
            impl DivAssign<$t> for StandardForm {
                fn div_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self /= rhs;
                }
            }
        )*
    };
    (rem => $($t : ty),*) => {
        $(
            impl Rem<$t> for StandardForm {
                type Output = Self;
                #[must_use]
                fn rem(self, other: $t) -> Self {
                    let rhs : Self = other.into();
                    self % rhs
                }
            }
            
            impl RemAssign<$t> for StandardForm {
                fn rem_assign(&mut self, other: $t) {
                    let rhs : Self = other.into();
                    *self %= rhs;
                }
            }
        )*
    };
    (pow => $($t : ty),*) => {
        $(
            #[cfg(feature="num")]
            impl num_traits::Pow<$t> for StandardForm {
                type Output = f64;
            
                #[must_use]
                fn pow(self, other: $t) -> Self::Output {
                    f64::from(self).powf(other as f64)
                }
            }
        )*
    };
    (operations => $($t:ty),*) => {
        $(
            primitives!(add => $t);
            primitives!(sub => $t);
            primitives!(mul => $t);
            primitives!(div => $t);
            primitives!(rem => $t);
            primitives!(pow => $t);
        )*
    }
}

primitives!(operations => i8, i16, i32, i64, u8, u16, u32, u64,f32,f64);
primitives!(form => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
primitives!(eq => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
primitives!(ord => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);

macro_rules! trig_functions {
    ($( {
        $(#[$attr:meta])* $fn : ident
    })*) => {
        #[cfg_attr(feature="js", wasm_bindgen)]
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


#[cfg(test)]
mod tests {
    use super::*;
    use Ordering;

    #[test]
    fn assignment_issue() {
        let sf1 = StandardForm::new(1.0,5);
        assert_eq!(*sf1.mantissa(),1.0);
        assert_eq!(*sf1.exponent(),5);
    }

    #[test]
    fn from_u8_standardform(){
        let n = 2u8;
        let r : StandardForm = n.into();

        assert_eq!(r,StandardForm { mantissa : 2.0,exponent : 0 });
    }

    #[test]
    fn test_normalize_with_valid_range() {
        let mut sf = StandardForm::new(2.5, 3);
        sf.adjust();
        assert_eq!(sf.mantissa, 2.5);
        assert_eq!(sf.exponent, 3);
    }

    #[test]
    fn test_normalize_with_invalid_range() {
        let mut sf = StandardForm::new(20.0, 3);
        sf.adjust();
        assert_eq!(sf.mantissa, 2.0);
        assert_eq!(sf.exponent, 4);
    }

    #[test]
    fn test_normalize_with_small_mantissa() {
        let mut sf = StandardForm::new(-0.25, 2);
        sf.adjust();
        assert_eq!(sf.mantissa, -2.5);
        assert_eq!(sf.exponent, 1);
    }

    #[test]
    fn test_normalize_with_large_negative_mantissa() {
        let mut sf = StandardForm::new(-750.0, 4);
        sf.adjust();
        assert_eq!(sf.mantissa, -7.5);
        assert_eq!(sf.exponent, 6);
    }

    #[test]
    fn addition() {
        // Test addition between StandardForm instances
        let a = StandardForm::new(1.2, 3);
        let b = StandardForm::new(3.4, 2);
        let result = a + b;
        assert_eq!(result, StandardForm::new(1.54,3) );
    }

    #[test]
    fn addition_u8() {
        // Test addition with u8
        let a = StandardForm::new(1.0, 1);
        let b = 2u8;
        let result = a + b;
        assert_eq!(result, StandardForm::new(1.2,1));
    }

    #[test]
    fn test_subtraction() {
        // Test subtraction between StandardForm instances
        let a = StandardForm::new(4.6, 2);
        let b = StandardForm::new(3.4, 2);
        let result = a - b;
        assert_eq!(result, StandardForm::new(1.2,2));
    }

    #[test]
    fn multiplication() {
        // Test multiplication between StandardForm instances
        let a = StandardForm::new(1.2, 3);
        let b = StandardForm::new(3.0, 2);
        let result = a * b;
        assert_eq!(result.mantissa, 3.6);
        assert_eq!(result.exponent, 5);
    }

    #[test]
    fn multiplication_u8() {
        // Test multiplication with u8
        let a = StandardForm::new(1.0, 1);        
        let b = 2u8;
        let result = a * b;
        assert_eq!(result.mantissa, 2.0);
        assert_eq!(result.exponent, 1);
    }

    #[test]
    fn division() {
        // Test division between StandardForm instances
        let a = StandardForm::new(4.0, 2);
        let b = StandardForm::new(2.0, 1);
        let result = a / b;
        assert_eq!(result.mantissa, 2.0);
        assert_eq!(result.exponent, 1);
    }

    #[test]
    fn division_u8() {
        // Test division with u8
        let a = StandardForm::new(2.0, 1);
        let b = 2u8;
        let result = a / b;
        assert_eq!(result.mantissa, 1.0);
        assert_eq!(result.exponent, 1);
    }


    #[test]
    fn add_assign() {
        let mut a = StandardForm::new(1.0, 1);
        let b = StandardForm::new(2.0, 1);
        a += b;
        assert_eq!(a.mantissa, 3.0);
        assert_eq!(a.exponent, 1);
    }

    #[test]
    fn add_assign_u8() {
        // Test AddAssign with u8
        let mut a = StandardForm::new(1.0, 1);

        let b = 2u8;

        a += b;
        assert_eq!(a.mantissa, 1.2);
        assert_eq!(a.exponent, 1);
    }

    #[test]
    fn test_partial_cmp_equal() {
        let sf1 = StandardForm::new(1.23, 3);
        let sf2 = StandardForm::new(1.23, 3);

        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Equal));
    }

    #[test]
    fn test_partial_cmp_greater() {

        //300
        let sf1 = StandardForm::new(3.0, 2);
        // 250
        let sf2 = StandardForm::new(2.5, 2);

        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Greater));
    }

    #[test]
    fn test_partial_cmp_less() {
        let sf1 = StandardForm::new(2.5, 2);
        let sf2 = StandardForm::new(3.0, 2);

        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Less));
    }

    #[test]
    fn test_partial_cmp_different_exponents() {
        let sf1 = StandardForm::new(1.5, 2);
        let sf2 = StandardForm::new(1.5, 3);

        // When exponents are different, the comparison is based on the magnitude
        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Less));
    }

    #[test]
    fn test_partial_cmp_zero() {
        let sf1 = StandardForm::new(0.0, 0);
        let sf2 = StandardForm::new(0.0, 0);

        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Equal));
    }

    #[test]
    fn test_partial_cmp_mixed_sign() {
        let sf1 = StandardForm::new(-1.0, 2);
        let sf2 = StandardForm::new(1.0, 2);

        // Negative numbers are considered less than positive numbers with the same magnitude
        assert_eq!(sf1.partial_cmp(&sf2), Some(Ordering::Less));
    }
}