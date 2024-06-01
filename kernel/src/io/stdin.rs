use alloc::vec::Vec;

use crate::interrupt::read_char;
use alloc::string::String;

// TODO make a stdin implementation that works like the stdlib one
pub fn read_line() -> String {
    let mut line = Vec::new();

    while line.last() != Some(&'\n') {
        let c = read_char();

        if c as u8 == 8 {
            line.pop();
        } else {
            line.push(c);
        }
    }

    line.into_iter().collect()
}
