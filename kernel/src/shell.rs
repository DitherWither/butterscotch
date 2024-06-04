use crate::{
    console_dbg, eprint, eprintln,
    fs::{self, ramfs},
    io, serial_dbg,
};
use alloc::string::{FromUtf8Error, String};
use alloc::vec::Vec;

pub fn run_shell() {
    let prompt = "$";
    let mut files = fs::ramfs::Directory::new();

    loop {
        match shell_inner(&prompt, &mut files) {
            None => {
                eprintln!("Some error occured");
            }
            Some(()) => {}
        }
    }
}

pub fn string_from_u8_nul_utf(utf8_src: &[u8]) -> Result<String, FromUtf8Error> {
    let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    String::from_utf8(utf8_src[0..nul_range_end].to_vec())
}

fn shell_inner(prompt: &str, files: &mut ramfs::Directory) -> Option<()> {
    eprint!("{prompt} ");
    let line_raw = io::stdin::read_line();
    let line: Vec<&str> = line_raw.split_whitespace().collect();
    let command = line[0];

    match command {
        "help" => {
            eprintln!("Currently available commands: help, echo, clear, put, cat, fsdump, mkdir")
        }
        "echo" => {
            // TODO remove command
            eprintln!("{}", line_raw);
        }

        "put" => {
            let contents = line[2];
            let mut file = files.create(line[1]).unwrap();

            file.write(contents.as_bytes()).unwrap();
        }

        "cat" => {
            let mut file = files.open(line[1], true).unwrap();
            let mut buf = [0u8; 1024];

            file.read(&mut buf).unwrap();

            eprintln!("{}", string_from_u8_nul_utf(&buf).unwrap());
        }
        "mkdir" => {
            files.mkdir(line[1]);
        }
        "fsdump" => {
            serial_dbg!(&files);
            console_dbg!(&files);
        }
        "clear" => {
            io::console::clear_screen();
        }
        _ => {
            eprintln!("Error: Unknown Command")
        }
    }

    Some(())
}
