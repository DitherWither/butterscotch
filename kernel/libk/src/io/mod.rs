pub mod stderr;
pub mod stdin;
pub mod stdout;
pub mod ramfile;

use crate::string::{String, ToString};
use crate::vec::Vec;
use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum Error {
    NegativeSeekError,
    IsDirectory,
    NotFound,
    InvalidPath,
    PermissionsError,
    InvalidString,
    ReadError,
    WriteError,
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub trait Seek {
    fn seek(&mut self, seek_from: SeekFrom) -> Result<u64, Error>;
}

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
