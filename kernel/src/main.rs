#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(butterscotch_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

// extern crate alloc;

// use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
// use bootloader::{entry_point, BootInfo};
use butterscotch_kernel::*;

// entry_point!(kernel_main);
// static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);
/// Sets the base revision to 1, this is recommended as this is the latest base revision described
/// by the Limine boot protocol specification. See specification for further info.
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new(1);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    butterscotch_kernel::init();

    // // allocate a number on the heap
    // let heap_value = Box::new(41);
    // println!("heap_value at {:p}", heap_value);

    // // create a dynamically sized vector
    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());

    // // create a reference counted vector -> will be freed when count reaches 0
    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = reference_counted.clone();
    // println!(
    //     "current reference count is {}",
    //     Rc::strong_count(&cloned_reference)
    // );
    // core::mem::drop(reference_counted);
    // println!(
    //     "reference count is {} now",
    //     Rc::strong_count(&cloned_reference)
    // );

    #[cfg(test)]
    test_main();

    eprintln!("Kernel did not crash");

    hlt_loop()
}

#[test_case]
#[allow(clippy::eq_op)]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", info);
    hlt_loop()
}
