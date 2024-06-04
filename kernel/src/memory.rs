use libk::Mutex;
use limine::{MemmapEntry, MemoryMapEntryType, NonNullPtr};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable,
        PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use crate::limine_requests::{HHDM_REQUEST, MEMMAP_REQUEST};

pub static PAGE_ALLOCATOR: Mutex<Option<PageAllocator>> = Mutex::new(None);

// TODO add 2mib

/// # Safety
///
/// Caller must ensure that the limine requests are valid,
/// and come from the bootloader
pub unsafe fn init() {
    let mut page_allocator = PAGE_ALLOCATOR.lock();
    if !page_allocator.is_none() {
        return;
    }

    *page_allocator = Some(PageAllocator::init(&MEMMAP_REQUEST, &HHDM_REQUEST));
}

pub struct PageAllocator<'a> {
    mapper: OffsetPageTable<'a>,
    frame_allocator_4kib: FrameAllocator4KiB<'a>,
}

impl<'a> PageAllocator<'a> {
    /// Marked private to avoid accidentally constructing multiple instances
    ///
    /// # Safety
    ///
    /// Caller must ensure that the limine requests are valid,
    /// and come from the bootloader
    unsafe fn init(
        memmap_request: &limine::MemmapRequest,
        hhdm_request: &limine::HhdmRequest,
    ) -> Self {
        // Get memmap
        let memmap = unsafe {
            memmap_request
                .get_response()
                .as_ptr()
                .expect("Unable to get memory map")
                .as_mut()
                .unwrap()
        }
        .memmap_mut();

        // Get physical memory offset
        let physical_memory_offset = hhdm_request.get_response().get().unwrap().offset;
        let physical_memory_offset = VirtAddr::new(physical_memory_offset);

        Self {
            mapper: Self::init_mapper(physical_memory_offset),
            frame_allocator_4kib: FrameAllocator4KiB::new(memmap),
        }
    }

    pub fn allocate_pages_4kib(
        &mut self,
        start: VirtAddr,
        end: VirtAddr,
        flags: PageTableFlags,
    ) -> Result<(), MapToError<Size4KiB>> {
        let start_page: Page<Size4KiB> = Page::containing_address(start);
        let end_page: Page<Size4KiB> = Page::containing_address(end);

        let page_range = Page::range_inclusive(start_page, end_page);

        for page in page_range {
            let frame = self
                .frame_allocator_4kib
                .allocate_frame()
                .ok_or(MapToError::FrameAllocationFailed)?;
            unsafe {
                self.mapper
                    .map_to_with_table_flags(
                        page,
                        frame,
                        flags,
                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                        &mut self.frame_allocator_4kib,
                    )?
                    .flush();
            }
        }

        Ok(())
    }
}

impl<'a> PageAllocator<'a> {
    /// # Safety
    ///
    /// Marked as safe to limit scope of unsafe, as this is a private function
    /// called by an unsafe function
    ///
    /// Caller must ensure that the entire physical memory is mapped to the virtual
    /// memory at the physical_memory_offset. This function should only be called once
    fn init_mapper(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
        let level_4_table = Self::active_level_4_table(physical_memory_offset);

        unsafe { OffsetPageTable::new(level_4_table, physical_memory_offset) }
    }

    /// Returns the active level 4 page table
    ///
    /// Marked as safe to limit scope of unsafe, as this is a private function
    /// called by an unsafe function
    ///
    /// # Safety
    /// Caller must ensure that the entire physical memory is mapped to the virtual
    /// memory at the physical_memory_offset. This function should only be called once
    /// to avoid multiple mutable references to the same memory
    fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
        let (level_4_table_frame, _) = Cr3::read();

        let physical_address = level_4_table_frame.start_address();
        let virtual_address = physical_memory_offset + physical_address.as_u64();

        let page_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();

        unsafe { &mut *page_table_ptr }
    }
}

/// Allocates memory frames
pub struct FrameAllocator4KiB<'a> {
    memory_map: &'a mut [NonNullPtr<MemmapEntry>],
    next: usize,
}

impl<'a> FrameAllocator4KiB<'a> {
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

unsafe impl FrameAllocator<Size4KiB> for FrameAllocator4KiB<'_> {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next); // TODO store usable frames somewhere
        self.next += 1;
        frame
    }
}
