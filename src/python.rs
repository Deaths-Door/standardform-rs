use super::*;

use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;
use num_traits::*;

#[allow(dead_code)]
fn standardform(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StandardForm>()?;
    Ok(())
}

pyo3::create_exception!(
    "standardform",
    PyConversionError,
    PyException,
    "A wrapper around the rust ConversionError enum"
);

impl PyConversionError {
    fn from(value: ConversionError) -> PyErr {
        Self::new_err(value.to_string())
    }
}

#[pymethods]
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
    #[new]
    // This is a 'replacement' for the Defatul trait
    #[pyo3(signature = (mantissa=1.0,exponent=0))]
    pub fn new_py(mantissa: f64, exponent: i8) -> PyResult<Self> {
        fn is_valid(float: f64) -> Option<ConversionError> {
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
            Some(error) => Err(PyConversionError::from(error)),
            None => Ok(Self::new(Finite::from_inner(mantissa), exponent)),
        }
    }

    #[pyo3(name = "to_scientific_notation")]
    fn to_scientific_notation_py(&self) -> String {
        format!("{}e{}", self.mantissa, self.exponent)
    }

    /// Returns the string representation of the number in engineering notation.
    #[must_use]
    #[pyo3(name = "to_engineering_notation")]
    fn to_engineering_notation_py(&self) -> String {
        format!("{}*10^{}", self.mantissa, self.exponent)
    }

    fn __hash__(&self) -> u64 {
        use core::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    // Debug
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __bool__(&self) -> bool {
        use num_traits::Zero;
        self.is_zero()
    }

    fn __float__(&self) -> f64 {
        self.clone().as_finite().into_inner()
    }

    fn __pos__(&self) -> Self {
        use num_traits::Signed;
        // Same Thing
        self.abs()
    }

    fn __neg__(&self) -> Self {
        -(self.clone())
    }

    fn __abs__(&self) -> Self {
        use num_traits::Signed;
        self.abs()
    }

    fn __add__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() + try_all_valid_types(rhs)?)
    }
    fn __iadd__<'py>(&mut self, rhs: &Bound<'py, PyAny>) -> PyResult<()> {
        *self += try_all_valid_types(rhs)?;
        Ok(())
    }
    fn __radd__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() + try_all_valid_types(rhs)?)
    }
    fn __sub__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() - try_all_valid_types(rhs)?)
    }
    fn __isub__<'py>(&mut self, rhs: &Bound<'py, PyAny>) -> PyResult<()> {
        *self -= try_all_valid_types(rhs)?;
        Ok(())
    }
    fn __rsub__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() - try_all_valid_types(rhs)?)
    }
    fn __mul__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() * try_all_valid_types(rhs)?)
    }
    fn __imul__<'py>(&mut self, rhs: &Bound<'py, PyAny>) -> PyResult<()> {
        *self *= try_all_valid_types(rhs)?;
        Ok(())
    }
    fn __rmul__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() * try_all_valid_types(rhs)?)
    }
    fn __mod__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() % try_all_valid_types(rhs)?)
    }
    fn __imod__<'py>(&mut self, rhs: &Bound<'py, PyAny>) -> PyResult<()> {
        *self %= try_all_valid_types(rhs)?;
        Ok(())
    }
    fn __rmod__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() % try_all_valid_types(rhs)?)
    }
    fn __div__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() / try_all_valid_types(rhs)?)
    }
    fn __idiv__<'py>(&mut self, rhs: &Bound<'py, PyAny>) -> PyResult<()> {
        *self /= try_all_valid_types(rhs)?;
        Ok(())
    }
    fn __rdiv__<'py>(&self, rhs: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(self.clone() / try_all_valid_types(rhs)?)
    }

    #[staticmethod]
    #[pyo3(name = "max_value")]
    fn max_value_py() -> Self {
        Self::max_value()
    }

    #[staticmethod]
    #[pyo3(name = "min_value")]
    fn min_value_py() -> Self {
        Self::min_value()
    }
    #[pyo3(name = "is_negative")]
    fn is_negative_py(&self) -> bool {
        self.is_negative()
    }
    #[pyo3(name = "is_positive")]
    fn is_positive_py(&self) -> bool {
        self.is_positive()
    }
    #[pyo3(name = "signum")]
    fn signum_py(&self) -> Self {
        self.signum()
    }
    #[pyo3(name = "abs_sub")]
    fn abs_sub_py(&self, other: &Self) -> Self {
        self.abs_sub(other)
    }
    #[pyo3(name = "abs")]
    fn abs_py(&self) -> Self {
        self.abs()
    }

    #[staticmethod]
    #[pyo3(name = "one")]
    fn one_py() -> Self {
        Self::one()
    }
    #[pyo3(name = "is_zero")]
    fn is_zero_py(&self) -> bool {
        self.is_zero()
    }

    #[staticmethod]
    #[pyo3(name = "zero")]
    fn zero_py() -> Self {
        Self::zero()
    }
}

fn try_all_valid_types<'py>(rhs: &Bound<'py, PyAny>) -> PyResult<StandardForm> {
    let output: StandardForm = if let Ok(rhs) = rhs.extract::<StandardForm>() {
        rhs.into()
    } else if let Ok(rhs) = rhs.extract::<i64>() {
        rhs.into()
    } else if let Ok(rhs) = rhs.extract::<i64>() {
        rhs.into()
    } else if let Ok(rhs) = rhs.extract::<f64>() {
        Finite::<f64>::from_inner(rhs).into()
    } else {
        // TODO; improve message
        return Err(PyValueError::new_err("rhs is not addable with self"));
    };

    Ok(output)
}
