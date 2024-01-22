#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(butterscotch_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use butterscotch_kernel::{memory, *};
use x86_64::{
    structures::paging::{FrameAllocator, Page, Translate},
    VirtAddr,
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    butterscotch_kernel::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe {memory::BootInfoFrameAllocator::new(&boot_info.memory_map)};

    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(100).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    eprintln!("Kernel did not crash");

    hlt_loop()
}

#[test_case]
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
