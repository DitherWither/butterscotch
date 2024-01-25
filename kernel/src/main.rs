#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(lazy_cell)]
#![feature(custom_test_frameworks)]
#![test_runner(butterscotch_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod interrupt;
pub mod io;
pub mod kernel;
pub use kernel::init;
pub mod kernel_allocator;
pub mod memory;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    init();

    eprintln!("Kernel did not crash");

    hlt_loop()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    hlt_loop()
}
