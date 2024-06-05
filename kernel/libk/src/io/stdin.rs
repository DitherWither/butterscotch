use super::{Error, Read};
use crate::hlt::hlt;
use crate::print;
use crate::string::String;
use crate::vec::Vec;
use spin::Mutex;

#[derive(Clone)]
pub struct Stdin;

unsafe impl Send for Stdin {}
unsafe impl Sync for Stdin {}

impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        buf[0] = getchar().try_into().map_err(|_| Error::ReadError)?;
        Ok(1)
    }
}

impl Stdin {
    pub fn read_line(&self, buf: &mut String) -> Result<usize, Error> {
        let mut line = Vec::new();

        while line.last() != Some(&'\n') {
            let c = getchar();
            if c as u8 == 8 {
                line.pop();
            } else {
                line.push(c);
            }
        }

        *buf = line.into_iter().collect();
        Ok(buf.len())
    }
}

pub fn stdin() -> Stdin {
    Stdin
}

// Will be set by the interrupt handler
pub static CUR_CHAR: Mutex<Option<char>> = Mutex::new(None);

// TODO this is platform specific code and should probably be in a separate module
pub fn getchar() -> char {
    while CUR_CHAR.lock().is_none() {
        hlt();
    }

    let c = CUR_CHAR.lock().unwrap();
    *CUR_CHAR.lock() = None;

    print!("{c}");
    c
}
