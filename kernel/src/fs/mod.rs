pub mod ramfs;

use crate::alloc::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;
use snafu::Snafu;

#[derive(Snafu, Debug)]
pub enum FileError {
    NegativeSeekError,
    IsDirectory,
    NotFound,
    InvalidPath,
    PermissionsError,
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

pub trait Directory {
    fn mkdir<T>(&self, path: T) -> Result<(), FileError>
    where
        T: Into<Path>;

    fn open<T>(&self, path: T, read_only: bool) -> Result<impl File, FileError>
    where
        T: Into<Path>;

    fn create<T>(&mut self, path: T) -> Result<impl File, FileError>
    where
        T: Into<Path>;
}

pub trait File: Read + Write + Seek {}
impl<T> File for T where T: Read + Write + Seek {}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, FileError>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, FileError>;
    fn flush(&mut self) -> Result<(), FileError>;
}

#[derive(Debug)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub trait Seek {
    fn seek(&mut self, seek_from: SeekFrom) -> Result<u64, FileError>;
}
