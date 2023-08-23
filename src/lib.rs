#![doc = include_str!("../README.md")]

#![forbid(
        missing_docs,
        unsafe_code,
        unused_imports,
        unused_variables,
        unused_mut,
        unused_results,
        unused_allocation,
        unused_must_use,
        unreachable_patterns,
        trivial_casts,
        unsafe_op_in_unsafe_fn,
        overflowing_literals,
)]

mod sf;
mod error;

pub use self::sf::*;
pub use self::error::*;