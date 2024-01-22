use limine::{MemmapEntry, MemoryMapEntryType, NonNullPtr};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
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

/// Allocates memory frames
pub struct BootInfoFrameAllocator<'a> {
    memory_map: &'a mut [NonNullPtr<MemmapEntry>],
    next: usize,
}

impl<'a> BootInfoFrameAllocator<'a> {
    /// # Safety
    ///
    /// Caller must ensure that the memory map is valid
    /// All frames that are marked as `USABLE` should really
    /// be unused
    pub unsafe fn new(memory_map: &'a mut [NonNullPtr<MemmapEntry>]) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }

    pub fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> + '_ {
        self.memory_map
            .iter()
            // Get usable reigions from the memory map
            .filter(|e| e.typ == MemoryMapEntryType::Usable)
            // Convert the regions to slices
            .map(|e| e.base..(e.base + e.len))
            // Bootloader page-aligns all usable memory, we don't need alignment or rounding code here
            // Create iterator of the usable ranges that steps by 4096 bytes
            .flat_map(|e| e.step_by(4096))
            // Create PhysFrame from the addresses
            .map(|e| PhysFrame::containing_address(PhysAddr::new(e)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator<'_> {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next); // TODO store usable frames somewhere
        self.next += 1;
        frame
    }
}
