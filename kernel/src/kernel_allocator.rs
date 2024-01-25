use spin::Mutex;
use talc::*;
use x86_64::{
    instructions::interrupts::without_interrupts,
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use crate::{memory};

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ErrOnOom> = Talc::new(ErrOnOom).lock();
static HEAP_SIZE: Mutex<u64> = Mutex::new(0);

// Start the heap at a this address to make it easier to recognize
pub const HEAP_START: usize = 0x_C444_4444_0000;
pub const HEAP_DEFAULT_SIZE: usize = 8 * 1024 * 1024; // 8KiB

// TODO: Use 2MiB pages instead for better performance
pub fn init() -> Result<(), MapToError<Size4KiB>> {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = heap_start + HEAP_DEFAULT_SIZE - 1u64;
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    if let Some(page_allocator) = &mut *memory::PAGE_ALLOCATOR.lock() {
        page_allocator.allocate_pages_4kib(heap_start, heap_end, flags)?;
    }

    unsafe {
        ALLOCATOR
            .lock()
            .claim(Span::from_base_size(
                heap_start.as_mut_ptr(),
                HEAP_DEFAULT_SIZE,
            ))
            .expect("Unable to claim heap");
    }

    *HEAP_SIZE.lock() = HEAP_DEFAULT_SIZE as u64;

    Ok(())
}