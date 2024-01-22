use talc::*;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ErrOnOom> = Talc::new(ErrOnOom).lock();

// Start the heap at a this address to make it easier to recognize
pub const HEAP_START: usize = 0x_C444_4444_0000;
// Increase this later once we need more than 8MiB
pub const HEAP_SIZE: usize = 8 * 1024 * 1024; // 8MiB

// TODO: Use 2MiB pages instead for better performance
pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = heap_start + HEAP_SIZE - 1u64;
    let heap_start_page = Page::containing_address(heap_start);
    let heap_end_page = Page::containing_address(heap_end);
    let page_range = Page::range_inclusive(heap_start_page, heap_end_page);

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to_with_table_flags(page, frame, flags, 
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE 
            , frame_allocator)?.flush() }
    }
    unsafe {
        ALLOCATOR
            .lock()
            .claim(Span::from_base_size(heap_start.as_mut_ptr(), HEAP_SIZE))
            .expect("Unable to claim heap");
    }

    Ok(())
}
