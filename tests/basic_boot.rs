#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(butterscotch_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use butterscotch_kernel::{hlt_loop, println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop();
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    butterscotch_kernel::test_panic_handler(info);
}
