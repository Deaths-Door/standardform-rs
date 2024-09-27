use super::*;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[cfg_attr(feature = "python", pymodule)]
fn standardform(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StandardForm>()?;
    Ok(())
}

#[cfg(feature = "python")]
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
        // Check for NaN
        if value.is_nan() {
            return Err(ConversionError::NaN);
        }

        // Check for Infinity
        if value.is_infinite() {
            return Err(ConversionError::Infinity);
        }

        Ok(Self::new(Finite::from_inner(mantissa), exponent))
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
