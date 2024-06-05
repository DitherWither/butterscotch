pub mod ramfs;

use libk::io::{self, Path, Read, Seek, Write};

pub trait Directory {
    fn mkdir<T>(&self, path: T) -> Result<(), io::Error>
    where
        T: Into<Path>;

    fn open<T>(&self, path: T, read_only: bool) -> Result<impl File, io::Error>
    where
        T: Into<Path>;

    fn create<T>(&mut self, path: T) -> Result<impl File, io::Error>
    where
        T: Into<Path>;
}

pub trait File: Read + Write + Seek {}
impl<T> File for T where T: Read + Write + Seek {}
