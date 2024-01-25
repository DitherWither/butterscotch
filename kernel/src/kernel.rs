use crate::{
    constants::KERNEL_VERSION,
    io::{
        console::{self},
        framebuffer::{self},
    },
    *,
};

/// Performs early kernel initialization
pub fn init() {
    unsafe {
        interrupt::init();
        memory::init();
    }
    kernel_allocator::init();
    framebuffer::init();
    console::clear_screen();

    serial_println!(" :: Butterscotch OS {KERNEL_VERSION} :: ");
    serial_println!("Copyright 2024 Vardhan Patil");
    console_println!(" :: Butterscotch OS {KERNEL_VERSION} :: ");
    console_println!("Copyright 2024 Vardhan Patil");
}
