use crate::{eprint, eprintln, io};
use alloc::vec::Vec;

pub fn run_shell() {
    let prompt = "$";

    loop {
        shell_inner(&prompt);
    }
}

fn shell_inner(prompt: &str) -> Option<()> {
    eprint!("{prompt} ");
    let line_raw = io::stdin::read_line();
    let line: Vec<&str> = line_raw.split_whitespace().collect();
    let command = line[0];

    match command {
        "help" => {
            eprintln!("Currently available commands: help, echo, clear")
        }
        "echo" => {
            // TODO remove command
            eprintln!("{}", line_raw);
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
