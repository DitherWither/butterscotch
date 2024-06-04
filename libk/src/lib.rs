#![no_std]

extern crate alloc;
extern crate core;

pub mod io;

mod hlt;
mod utils;

pub use spin::{Mutex, MutexGuard};
