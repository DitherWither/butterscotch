#![no_std]

pub extern crate alloc;
pub extern crate core;

pub use alloc::{rc, slice, str, string, vec};
pub use core::*;

pub mod io;

mod hlt;
mod utils;

pub use hashbrown::{hash_map, hash_set, hash_table};
pub use spin::{Mutex, MutexGuard};
