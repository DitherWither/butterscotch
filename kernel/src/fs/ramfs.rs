use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::min;
use hashbrown::HashMap;
use snafu::Snafu;
use spin::Mutex;
use spin::MutexGuard;

#[derive(Snafu, Debug)]
pub enum FileError {
    NegativeSeekError,
    IsDirectory,
    NotFound,
    InvalidPath,
    PermissionsError,
}

#[derive(Debug)]
pub enum File {
    Directory(Directory),
    Regular(RegularFile),
}

#[derive(Debug)]
pub struct Directory {
    contents: Mutex<HashMap<String, File>>,
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            contents: Default::default(),
        }
    }
    fn _open(&self, path: &[&str], read_only: bool, create: bool) -> Result<OpenFile, FileError> {
        let mut contents = self.contents.lock();

        // Stupid hack to force rust into giving us multiple mutable refs
        // by casting to a pointer and casting back to a ref
        // TODO This is stupid, find a way to do it in safe rust
        let contents_ptr = &mut contents as *mut MutexGuard<HashMap<String, File>>;
        let contents = unsafe { &mut *contents_ptr };
        if path.len() == 0 {
            return Err(FileError::IsDirectory);
        }
        match contents.get(path[0]) {
            Some(file) => match file {
                File::Directory(dir) => dir._open(&path[1..], read_only, create),
                File::Regular(file) => Ok(file.open(read_only)),
            },
            None => {
                if path.len() != 1 {
                    return Err(FileError::InvalidPath);
                }
                if create {
                    let file = RegularFile::create();
                    let contents = unsafe { &mut *contents_ptr };
                    contents.insert(path[0].to_string(), File::Regular(file));
                    if let Some(File::Regular(file)) = contents.get(path[0]) {
                        Ok(file.open(read_only))
                    } else {
                        panic!("How did this even happen");
                    }
                } else {
                    Err(FileError::NotFound)
                }
            }
        }
    }

    pub fn mkdir(&self, path: &[&str]) -> Result<(), FileError> {
        let mut contents = self.contents.lock();
        if path.len() == 0 {
            return Ok(());
        }

        match contents.get(path[0]) {
            Some(file) => match file {
                File::Directory(dir) => dir.mkdir(&path[1..]),
                File::Regular(file) => Err(FileError::InvalidPath),
            },
            None => {
                contents.insert(path[0].to_string(), File::Directory(Directory::new()));
                // Run in case the path has more segments after this
                if path.len() > 1 {
                    if let Some(File::Directory(dir)) = contents.get(path[0]) {
                        dir.mkdir(&path[1..])
                    } else {
                        panic!("How did this happen");
                    }
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn open(&self, path: &[&str], read_only: bool) -> Result<OpenFile, FileError> {
        self._open(path, read_only, false)
    }

    pub fn create(&mut self, path: &[&str]) -> Result<OpenFile, FileError> {
        self._open(path, false, true)
    }
}

#[derive(Debug)]
pub struct RegularFile {
    contents: Mutex<Vec<u8>>,
}

impl RegularFile {
    pub fn create() -> RegularFile {
        RegularFile {
            contents: Default::default(),
        }
    }
    pub fn open(&self, read_only: bool) -> OpenFile {
        OpenFile {
            contents: &self.contents,
            position: 0,
            read_only,
        }
    }
}

#[derive(Debug)]
pub struct OpenFile<'a> {
    contents: &'a Mutex<Vec<u8>>,
    position: usize,
    read_only: bool,
}

#[derive(Debug)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

impl OpenFile<'_> {
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, FileError> {
        let contents = self.contents.lock();
        let end = min(self.position + buf.len(), contents.len());

        for (i, el) in contents[self.position..end].iter().enumerate() {
            buf[i] = *el;
        }
        buf[end - self.position] = 0;
        Ok(end - self.position)
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, FileError> {
        if self.read_only {
            return Err(FileError::PermissionsError);
        }

        let mut contents = self.contents.lock();
        for byte in buf {
            if self.position < contents.len() {
                contents[self.position] = *byte;
            } else {
                contents.push(*byte);
            }
            self.position += 1;
        }
        Ok(buf.len())
    }

    // RamFS is always flushed instantly
    pub fn flush(&mut self) -> Result<(), FileError> {
        Ok(())
    }

    // TODO: This code is horrible, refactor this
    pub fn seek(&mut self, seek_from: SeekFrom) -> Result<u64, FileError> {
        let len = self.contents.lock().len();
        match seek_from {
            SeekFrom::Start(n) => {
                self.position = n as usize;
                if self.position > len {
                    self.position -= len;
                }
                Ok(self.position as u64)
            }
            SeekFrom::End(n) => {
                let new_pos = len as i64 + n;
                if new_pos < 0 {
                    return Err(FileError::NegativeSeekError);
                }
                self.position = new_pos as usize;
                if self.position > len {
                    self.position -= len;
                }
                Ok(self.position as u64)
            }
            SeekFrom::Current(n) => {
                let new_pos = self.position as i64 + n;
                if new_pos < 0 {
                    return Err(FileError::NegativeSeekError);
                }
                self.position = new_pos as usize;
                if self.position > len {
                    self.position -= len;
                }
                Ok(self.position as u64)
            }
        }
    }
}
