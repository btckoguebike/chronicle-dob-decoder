#![cfg_attr(not(test), no_std)]

extern crate alloc;
mod error;

pub mod decoder;
pub mod generated;
pub mod handler;
pub mod object;
pub use error::Error::InvalidCombination;
