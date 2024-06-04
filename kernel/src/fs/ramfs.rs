use hashbrown::HashMap;
use libk::cmp::min;
use libk::string::{String, ToString};
use libk::vec::Vec;

use super::{Directory, File};
use libk::io::{self, Path, Read, Seek, SeekFrom, Write};
use libk::Mutex;
use libk::MutexGuard;

#[derive(Debug)]
pub enum RamFsNode {
    Directory(RamFsDirectory),
    Regular(RegularFile),
}

#[derive(Debug)]
pub struct RamFsDirectory {
    contents: Mutex<HashMap<String, RamFsNode>>,
}

impl RamFsDirectory {
    fn _open(
        &self,
        path: &[String],
        read_only: bool,
        create: bool,
    ) -> Result<RamFsFile, io::Error> {
        let mut contents = self.contents.lock();

        // Stupid hack to force rust into giving us multiple mutable refs
        // by casting to a pointer and casting back to a ref
        // TODO This is stupid, find a way to do it in safe rust
        let contents_ptr = &mut contents as *mut MutexGuard<HashMap<String, RamFsNode>>;
        let contents = unsafe { &mut *contents_ptr };
        if path.is_empty() {
            return Err(io::Error::IsDirectory);
        }
        match contents.get(&path[0]) {
            Some(file) => match file {
                RamFsNode::Directory(dir) => dir._open(&path[1..], read_only, create),
                RamFsNode::Regular(file) => Ok(file.open(read_only)),
            },
            None => {
                if path.len() != 1 {
                    return Err(io::Error::InvalidPath);
                }
                if create {
                    let file = RegularFile::create();
                    let contents = unsafe { &mut *contents_ptr };
                    contents.insert(path[0].to_string(), RamFsNode::Regular(file));
                    if let Some(RamFsNode::Regular(file)) = contents.get(&path[0]) {
                        Ok(file.open(read_only))
                    } else {
                        panic!("How did this even happen");
                    }
                } else {
                    Err(io::Error::NotFound)
                }
            }
        }
    }
}

impl RamFsDirectory {
    pub fn new() -> RamFsDirectory {
        RamFsDirectory {
            contents: Default::default(),
        }
    }
}

impl Default for RamFsDirectory {
    fn default() -> Self {
        Self::new()
    }
}

impl Directory for RamFsDirectory {
    fn mkdir<T>(&self, path: T) -> Result<(), io::Error>
    where
        T: Into<Path>,
    {
        let path: Path = path.into();
        let path = path.segments;
        let mut contents = self.contents.lock();
        if path.is_empty() {
            return Ok(());
        }

        match contents.get(&path[0]) {
            Some(file) => match file {
                RamFsNode::Directory(dir) => dir.mkdir(&path[1..]),
                RamFsNode::Regular(_) => Err(io::Error::InvalidPath),
            },
            None => {
                contents.insert(
                    path[0].to_string(),
                    RamFsNode::Directory(RamFsDirectory::new()),
                );
                // Run in case the path has more segments after this
                if path.len() > 1 {
                    if let Some(RamFsNode::Directory(dir)) = contents.get(&path[0]) {
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

    fn open<T>(&self, path: T, read_only: bool) -> Result<impl File, io::Error>
    where
        T: Into<Path>,
    {
        let path: Path = path.into();
        self._open(&path.segments, read_only, false)
    }

    fn create<T>(&mut self, path: T) -> Result<impl File, io::Error>
    where
        T: Into<Path>,
    {
        let path: Path = path.into();
        self._open(&path.segments, false, true)
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
    pub fn open(&self, read_only: bool) -> RamFsFile {
        RamFsFile {
            contents: &self.contents,
            position: 0,
            read_only,
        }
    }
}

#[derive(Debug)]
pub struct RamFsFile<'a> {
    contents: &'a Mutex<Vec<u8>>,
    position: usize,
    read_only: bool,
}

impl Read for RamFsFile<'_> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let contents = self.contents.lock();
        let end = min(self.position + buf.len(), contents.len());

        for (i, el) in contents[self.position..end].iter().enumerate() {
            buf[i] = *el;
        }
        buf[end - self.position] = 0;
        Ok(end - self.position)
    }
}

impl Write for RamFsFile<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        if self.read_only {
            return Err(io::Error::PermissionsError);
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
    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

impl Seek for RamFsFile<'_> {
    // TODO: This code is horrible, refactor this
    fn seek(&mut self, seek_from: SeekFrom) -> Result<u64, io::Error> {
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
                    return Err(io::Error::NegativeSeekError);
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
                    return Err(io::Error::NegativeSeekError);
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
