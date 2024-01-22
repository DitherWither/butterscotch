use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable,
        PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

/// Returns the active level 4 page table
///
/// # Safety
/// Caller must ensure that the entire physical memory is mapped to the virtual
/// memory at the physical_memory_offset. This function should only be called once
/// to avoid multiple mutable references to the same memory
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let physical_address = level_4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();

    let page_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();

    &mut *page_table_ptr
}

/// # Safety
///
/// Caller must ensure that the entire physical memory is mapped to the virtual
/// memory at the physical_memory_offset. This function should only be called once
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);

    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Creates a mapping to the vga text buffer, temporary testing function
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: this is not safe, we do it only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

/// Allocates memory frames
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// # Safety
    ///
    /// Caller must ensure that the memory map is valid
    /// All frames that are marked as `USABLE` should really
    /// be unused
    pub unsafe fn new(memory_map: &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }

    pub fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.memory_map
            .iter()
            // Get usable reigions from the memory map
            .filter(|e| e.region_type == MemoryRegionType::Usable)
            // Convert the regions to slices
            .map(|e| e.range.start_addr()..e.range.end_addr())
            // Bootloader page-aligns all usable memory, we don't need alignment or rounding code here
            // Create iterator of the usable ranges that steps by 4096 bytes
            .flat_map(|e| e.step_by(4096))
            // Create PhysFrame from the addresses
            .map(|e| PhysFrame::containing_address(PhysAddr::new(e)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next); // TODO store usable frames somewhere
        self.next += 1;
        frame
    }
}
