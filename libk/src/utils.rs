use alloc::string::{FromUtf8Error, String};

pub(crate) fn string_from_u8_nul_utf(utf8_src: &[u8]) -> Result<String, FromUtf8Error> {
    let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    String::from_utf8(utf8_src[0..nul_range_end].to_vec())
}
