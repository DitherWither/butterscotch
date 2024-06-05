use talc::*;
use x86_64::{structures::paging::PageTableFlags, VirtAddr};

use crate::{
    constants::{HEAP_DEFAULT_SIZE, HEAP_START},
    memory,
};

use libk::Mutex;

#[global_allocator]
static ALLOCATOR: Talck<Mutex<()>, ErrOnOom> = Talc::new(ErrOnOom).lock();

// TODO: Use 2MiB pages instead for better performance
/// Initialize the allocator
///
/// Called by kernel::init by default
pub fn init() {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = heap_start + HEAP_DEFAULT_SIZE - 1u64;
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    if let Some(page_allocator) = &mut *memory::PAGE_ALLOCATOR.lock() {
        page_allocator
            .allocate_pages_4kib(heap_start, heap_end, flags)
            .expect("Unable to allocate pages");
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
}
