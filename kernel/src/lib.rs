#![no_std]
#![cfg_attr(test, no_main)]
#![feature(const_mut_refs)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(lazy_cell)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

// extern crate alloc;

pub mod gdt;
pub mod interrupt;
pub mod io;
pub mod kernel;
pub use kernel::init;
pub mod allocator;
pub mod memory;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
pub fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[allow(dead_code)]
pub fn test_runner(tests: &[&dyn crate::Testable]) {
    serial_println!(" ******** Butterscotch OS 0.1.0 Alpha Testing Mode ********\n");
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// Exits qemu, for testing without getting APM or ACPI working
#[allow(dead_code, clippy::empty_loop)]
pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    serial_println!("Could not quit qemu! Make sure that tests are being run on qemu with the correct arguments");
    hlt_loop()
}

/// Panic handler for test mode
pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info);
}
