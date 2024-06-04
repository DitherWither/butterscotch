use core::fmt::Debug;

use crate::*;
use alloc::vec::Vec;
use libk::io::{Read, Write};
use libk::io::stdout::STDOUT;
use crate::fs::Directory;

pub fn run_shell() {
    println!();
    let prompt = "$";
    let mut files = fs::ramfs::RamFsDirectory::new();

    loop {
        match shell_inner(&prompt, &mut files) {
            None => {
                eprintln!("Some error occured");
            }
            Some(()) => {}
        }
    }
}



fn shell_inner(prompt: &str, files: &mut (impl Directory + Debug)) -> Option<()> {
    eprint!("{prompt} ");
    let line_raw = io::stdin::read_line();
    let line: Vec<&str> = line_raw.split_whitespace().collect();
    let command = line[0];

    match command {
        "help" => {
            println!("Currently available commands: help, echo, clear, put, cat, fsdump, mkdir")
        }
        "echo" => {
            // TODO remove command
            println!("{}", line_raw);
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

            STDOUT.lock().write(&buf).unwrap();
        }
        "mkdir" => {
            files.mkdir(line[1]).unwrap();
        }
        "fsdump" => {
            dbg!(&files);
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
