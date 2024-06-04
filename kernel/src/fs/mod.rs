pub mod ramfs;

use crate::alloc::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Path {
    pub segments: Vec<String>,
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self {
            segments: value
                .split('/')
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

impl From<&[String]> for Path {
    fn from(value: &[String]) -> Self {
        Self {
            segments: value.to_vec(),
        }
    }
}
