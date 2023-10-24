#![doc = include_str!("../README.md")]

#![forbid(
        missing_docs,
        unsafe_code,
        unused_imports,
        unused_mut,
        unused_results,
        unused_allocation,
        unused_must_use,
        unreachable_patterns,
        trivial_casts,
        unsafe_op_in_unsafe_fn,
        overflowing_literals,
)]

#![cfg_attr(
        not(feature = "std"), 
        no_std
)]

mod sf;
pub use self::sf::*;

// needed for thiserror::Error macro
#[cfg(feature = "std")]
mod error;

#[cfg(feature = "std")]
pub use self::error::*;

#[cfg(feature = "num")]
mod num;

#[cfg(feature = "num")]
pub use self::num::*;

#[cfg(feature = "hash")]
mod hash;

#[cfg(feature = "hash")]
pub use hash::*;

#[cfg(feature = "nom")]
mod nom;

#[cfg(feature = "nom")]
pub use nom::*;
