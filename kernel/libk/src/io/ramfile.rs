use crate::{
    cmp::min,
    io::{self, Read, Seek, SeekFrom, Write},
    vec::Vec,
    Mutex,
};

#[derive(Debug)]
pub struct RamFile<'a> {
    contents: &'a Mutex<Vec<u8>>,
    position: usize,
    read_only: bool,
}

impl<'a> RamFile<'a> {
    pub fn new(contents: &'a Mutex<Vec<u8>>, read_only: bool) -> Self {
        Self {
            contents,
            read_only,
            position: 0,
        }
    }
}

impl Read for RamFile<'_> {
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

impl Write for RamFile<'_> {
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

impl Seek for RamFile<'_> {
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
