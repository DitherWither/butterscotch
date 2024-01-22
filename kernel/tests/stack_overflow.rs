#![no_std]
#![no_main]

use butterscotch_kernel::{gdt, interrupt, serial_print};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    unsafe {
        gdt::init();
        interrupt::_init(true);
    }
    serial_print!("stack_overflow::stack_overflow...\t");

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    butterscotch_kernel::test_panic_handler(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
}
