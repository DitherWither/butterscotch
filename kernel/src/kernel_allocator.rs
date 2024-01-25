use spin::Mutex;
use talc::*;
use x86_64::{
    structures::paging::{
        mapper::MapToError, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use crate::memory;

#[global_allocator]
static ALLOCATOR: Talc<spin::Mutex<()>, ErrOnOom> = Talc::new(ErrOnOom).lock();
static HEAP_SIZE: Mutex<u64> = Mutex::new(0);

// Constants
const HEAP_START: usize = 0xC444_4444_0000;
const HEAP_DEFAULT_SIZE: usize = 8 * 1024 * 1024; // 8 MiB

// TODO: Use 2 MiB pages instead for better performance
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
            .map_err(|err| {
                eprintln!("Unable to claim heap: {:?}", err);
                err
            })?;
    }

    *HEAP_SIZE.lock() = HEAP_DEFAULT_SIZE as u64;

    Ok(())
}
