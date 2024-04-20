#![cfg_attr(not(test), no_std)]

extern crate alloc;
mod core;
mod error;
mod generated;
mod object;

pub mod handler;
