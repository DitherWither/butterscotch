use bootloader::BootInfo;
use x86_64::VirtAddr;

use crate::*;

pub fn init(boot_info: &'static BootInfo) {
    unsafe {
        gdt::init();
        interrupt::init();
    }

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::new(&boot_info.memory_map) };
    allocator::init(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    println!(" :: Butterscotch OS 0.1.0 Alpha :: ");
    serial_println!(" :: Butterscotch OS 0.1.0 Alpha :: ");

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );
}
