use alloc::vec::Vec;

use crate::interrupt::read_char;
use alloc::string::String;

// TODO make a stdin implementation that works like the stdlib one
pub fn read_line() -> String{
    let mut line = Vec::new();

    while line.last() != Some(&'\n') {
        line.push(read_char());
    }
    
    line.into_iter().collect()
}