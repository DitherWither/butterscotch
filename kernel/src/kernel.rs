use libk::println;
use crate::{
    constants::KERNEL_VERSION,
    io::{
        console::{self},
        framebuffer::{self},
    },
    *,
};
use crate::io::console::CONSOLE;
use crate::io::serial;
use crate::io::serial::SERIAL1;

/// Performs early kernel initialization
pub fn init() {
    unsafe {
        interrupt::init();
        memory::init();
    }
    kernel_allocator::init();
    framebuffer::init();
    console::clear_screen();
    serial::init();

    libk::io::stdout::add_sink(&CONSOLE);
    libk::io::stderr::add_sink(&CONSOLE);

    libk::io::stdout::add_sink(&SERIAL1);
    libk::io::stderr::add_sink(&SERIAL1);

    println!(" :: Butterscotch OS {KERNEL_VERSION} :: ");
    println!("Copyright 2024 Vardhan Patil");
}
