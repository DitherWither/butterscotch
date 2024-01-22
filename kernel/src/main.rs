#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(lazy_cell)]
#![feature(custom_test_frameworks)]
#![test_runner(butterscotch_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod gdt;
pub mod interrupt;
pub mod io;
pub mod kernel;
pub use kernel::init;
pub mod allocator;
pub mod memory;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    init();

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    eprintln!("Kernel did not crash");

    hlt_loop()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    hlt_loop()
}
