#![allow(missing_docs)]

use super::*;
use num_traits::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
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
    #[wasm_bindgen(constructor)]
    pub fn  new_wasm(mantissa: f64, exponent: i8) -> Result<StandardForm,ConversionError> {
        fn  is_valid(float: f64) -> Option<ConversionError> {
            // Check for NaN
            if float.is_nan() {
                return Some(ConversionError::NaN);
            }

            // Check for Infinity
            if float.is_infinite() {
                return Some(ConversionError::Infinity);
            }

            None
        }

        match is_valid(mantissa) {
            Some(error) => Err(error),
            None => Ok(Self::new(Finite::from_inner(mantissa), exponent)),
        }
    }


    #[wasm_bindgen(js_name="max_value")] pub fn  max_value_wasm() -> Self { Self::max_value() }
    #[wasm_bindgen(js_name="min_value")] pub fn  min_value_wasm() -> Self { Self::min_value() }
    #[wasm_bindgen(js_name="is_negative")] pub fn  is_negative_wasm(&self) -> bool { self.is_negative() }
    #[wasm_bindgen(js_name="is_positive")] pub fn  is_positive_wasm(&self) -> bool { self.is_positive() }
    #[wasm_bindgen(js_name="signum")] pub fn  signum_wasm(&self) -> Self { self.signum() }
    #[wasm_bindgen(js_name="abs_sub")] pub fn  abs_sub_wasm(&self, other: &StandardForm) -> Self { self.abs_sub(other) }
    #[wasm_bindgen(js_name="abs")] pub fn  abs_wasm(&self) -> Self { self.abs() }
    #[wasm_bindgen(js_name="one")] pub fn  one_wasm() -> Self { Self::one() }
    #[wasm_bindgen(js_name="is_zero")] pub fn  is_zero_wasm(&self) -> bool { self.is_zero() }
    #[wasm_bindgen(js_name="zero")] pub fn  zero_wasm() -> Self { Self::zero() }
    #[wasm_bindgen(js_name="add")] pub fn  add_wasm(self,rhs : Self) -> Self { self + rhs }
    #[wasm_bindgen(js_name="add_assign")] pub fn  add_assign_wasm(&mut self,rhs : Self) { *self+= rhs  }
    #[wasm_bindgen(js_name="sub")] pub fn  sub_wasm(self,rhs : Self) -> Self { self - rhs }
    #[wasm_bindgen(js_name="sub_assign")] pub fn  sub_assign_wasm(&mut self,rhs : Self) { *self-= rhs  }
    #[wasm_bindgen(js_name="mul")] pub fn  mul_wasm(self,rhs : Self) -> Self { self * rhs }
    #[wasm_bindgen(js_name="mul_assign")] pub fn  mul_assign_wasm(&mut self,rhs : Self) { *self*= rhs  }
    #[wasm_bindgen(js_name="div")] pub fn  div_wasm(self,rhs : Self) -> Self { self / rhs }
    #[wasm_bindgen(js_name="div_assign")] pub fn  div_assign_wasm(&mut self,rhs : Self) { *self/= rhs  }
}