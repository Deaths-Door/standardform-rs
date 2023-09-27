use std::hash::{Hash, Hasher};
use ordered_float::OrderedFloat;

use crate::StandardForm;

impl Hash for StandardForm { 
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        OrderedFloat(*self.mantissa()).hash(state);
        self.exponent().hash(state);
    }
}