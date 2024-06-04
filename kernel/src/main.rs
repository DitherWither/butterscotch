#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(lazy_cell)]
#![feature(custom_test_frameworks)]
#![test_runner(butterscotch_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod constants;
pub mod fs;
pub mod interrupt;
pub mod io;
pub mod kernel;
pub mod kernel_allocator;
pub mod limine_requests;
pub mod memory;
pub mod shell;

pub use kernel::init;
use shell::run_shell;

pub use libk::{dbg, eprint, eprintln, print, println};

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    init();

    run_shell();

    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    hlt_loop()
}
