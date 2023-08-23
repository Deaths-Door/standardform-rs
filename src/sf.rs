use crate::ParsingStandardFormError;

use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};
use std::cmp::max;

use arkley_traits::Power;

/// Represents a number in standard form.
///
/// The `Standardform` struct holds the significand (mantissa) of the number 
/// and an exponent that determines the power of 10 by which the significand should be multiplied.
#[derive(Clone, PartialEq)]
pub struct StandardForm  {
    mantissa : f64,
    exponent : i8
}

impl StandardForm {
    /// Creates a new instance of `StandardForm` with the given mantissa and exponent.
    ///
    /// This constructor initializes a new `StandardForm` instance with the provided `mantissa` and `exponent`.
    /// It's important to note that the provided `mantissa` and `exponent` may not be exactly the same as the
    /// values stored in the resulting instance. The values are adjusted automatically to adhere to the rules
    /// of standard form representation, ensuring the most appropriate form for the given input.
    pub fn new(mantissa : f64,exponent : i8) -> Self {
        let mut instance = Self { mantissa , exponent};
        instance.adjust();
        instance
    }

    fn in_range(&self) -> bool {
        (self.mantissa >= 1.0 && self.mantissa <= 10.0) || (self.mantissa >= -10.0 && self.mantissa <= -1.0)
    }

    /// Adjusts the mantissa and exponent of the `StandardForm` instance to adhere to standard form representation rules.
    ///
    /// If the current mantissa and exponent do not satisfy the standard form representation requirements,
    /// this method will adjust them while maintaining the value of the number represented. The adjustment
    /// ensures that the mantissa is between 1 (inclusive) and 10 (exclusive) and the exponent is such that
    /// the product of mantissa and 10 raised to the exponent yields the original number.
    ///
    /// This adjustment is particularly useful when initializing a `StandardForm` instance with arbitrary values,
    /// as it ensures that the instance accurately represents the provided numerical value in standard form.
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

    /// Returns a reference to the StandardForm representing the significand (mantissa) of the number.
    pub const fn mantissa(&self) -> &f64 {
        &self.mantissa
    }

    /// Returns the exponent that determines the power of 10 by which the significand should be multiplied.
    pub const fn exponent(&self) -> &i8 {
        &self.exponent
    }

    /// Returns the string representation of the number in scientific notation.
    pub fn to_scientific_notation(&self) -> String {
        format!("{}e{}", self.mantissa, self.exponent)
    }
        
    /// Returns the string representation of the number in engineering notation.
    pub fn to_engineering_notation(&self) -> String {
        format!("{}*10^{}", self.mantissa, self.exponent)
    }
}

impl Default for StandardForm {
    fn default() -> Self {
        Self { mantissa : 1.0, exponent : 0 }
    }
}

impl PartialOrd for StandardForm {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.exponent == other.exponent {
            true => self.mantissa.partial_cmp(&other.mantissa),
            false => self.exponent.partial_cmp(&other.exponent)
        }
    }
}

impl std::fmt::Display for StandardForm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.exponent > 4 {
            return write!(f,"{}",self.to_scientific_notation());
        };

        write!(f,"{}",self.mantissa * 10_i32.pow(self.exponent as u32) as f64)
    }
}

impl TryFrom<&str> for StandardForm {
    type Error = ParsingStandardFormError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(number) = value.parse::<f64>() {
            return Ok(number.into());
        }

        if let Some(index) = value.find('e') {
            let m_str : f64 = value[0..index].parse().map_err(|error| ParsingStandardFormError::Mantissa(error) )?;
            let e_str : i8 = value[index + 1..].parse().map_err(|error| ParsingStandardFormError::Exponent(error) )?;
            return Ok(StandardForm::new(m_str,e_str));
        }
        
        if let Some(index) = value.find('^') {
            let m_str : f64 = value[0..index - 2].parse().map_err(|error| ParsingStandardFormError::Mantissa(error) )?;
            let e_str : i8 = value[index + 1..].parse().map_err(|error| ParsingStandardFormError::Exponent(error) )?;
            return Ok(StandardForm::new(m_str,e_str));
        }

        Err(ParsingStandardFormError::InvalidFormat)
    }
}

impl Add for StandardForm {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let max_power = max(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0_f64.to_the_power_of((self.exponent - max_power) as f64) + other.mantissa * 10.0_f64.to_the_power_of((other.exponent - max_power) as f64);
        StandardForm::new(num_sum, max_power)
    }
}

impl AddAssign for StandardForm {
    fn add_assign(&mut self, other: Self) {
        let max_power = max(self.exponent, other.exponent);
        let num_sum = self.mantissa * 10.0_f64.to_the_power_of((self.exponent - max_power) as f64) + other.mantissa * 10.0_f64.to_the_power_of((other.exponent - max_power) as f64);

        self.mantissa = num_sum;
        self.exponent = max_power;

        self.adjust();
    }
}

impl Sub for StandardForm {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let min = self.exponent.min(other.exponent);

        let x = self.mantissa * 10_i32.pow((self.exponent - min) as u32) as f64;
        let y = other.mantissa * 10_i32.pow((other.exponent - min) as u32) as f64;

        let result = x - y;
        let rounded = (result * 1.0e6).round() / 1.0e6;

        StandardForm::new(rounded,min)
    }
}

impl SubAssign for StandardForm {
    fn sub_assign(&mut self, other: Self) {
        let min = self.exponent.min(other.exponent);

        let x = self.mantissa * 10_i32.pow((self.exponent - min) as u32) as f64;
        let y = other.mantissa * 10_i32.pow((other.exponent - min) as u32) as f64;

        let result = x - y;
        let rounded = (result * 1.0e6).round() / 1.0e6;

        self.mantissa = rounded;
        self.exponent = min;
        self.adjust(); 
    }
}

impl Mul for StandardForm {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let exponent = self.exponent + other.exponent;
        let mantissa = self.mantissa * other.mantissa;
        let rounded = (mantissa * 1.0e6).round() / 1.0e6;
        StandardForm::new(rounded,exponent)
    }
}

impl MulAssign for StandardForm {
    fn mul_assign(&mut self, other: Self) {
        let exponent = self.exponent + other.exponent;
        let mantissa = self.mantissa * other.mantissa;
        let rounded = (mantissa * 1.0e6).round() / 1.0e6;

        self.mantissa = rounded;
        self.exponent = exponent;

        self.adjust();
    }
}

impl Div for StandardForm {
    type Output = Self;
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

macro_rules! primitives {
    (form => $($t:ty),*) => {
        $(
            impl From<$t> for StandardForm {
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
                fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
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
    (operations => $($t:ty),*) => {
        $(
            primitives!(add => $t);
            primitives!(sub => $t);
            primitives!(mul => $t);
            primitives!(div => $t);
        )*
    }
}

primitives!(operations => i8, i16, i32, i64, u8, u16, u32, u64,f32,f64);
primitives!(form => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
primitives!(eq => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
primitives!(ord => u8,u16,u32,u64,i8,i16,i32,i64,f32,f64);
