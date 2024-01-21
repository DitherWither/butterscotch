#![no_std]
#![no_main]
mod vga_text_buffer;

use core::panic::PanicInfo;

// use vga_text_buffer::print_something;
use vga_text_buffer::CONSOLE;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // print_something();
    CONSOLE.lock().write_string("Hello, World!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
