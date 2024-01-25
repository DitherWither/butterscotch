use x86_64::instructions::interrupts::int3;

use crate::{
    io::{
        console::{self},
        framebuffer::{self},
    },
    *,
};
pub static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);
/// Sets the base revision to 1, this is recommended as this is the latest base revision described
/// by the Limine boot protocol specification. See specification for further info.
static _BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new(1);

static MEMMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(1);
// let mut mapper = unsafe { memory::init(physical_memory_offset) };
// let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::new(&boot_info.memory_map) };
// allocator::init(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

static HHDM_REQUEST: limine::HhdmRequest = limine::HhdmRequest::new(1);

pub fn init() {
    unsafe {
        interrupt::init();
        memory::init(&MEMMAP_REQUEST, &HHDM_REQUEST);
    }
    kernel_allocator::init().expect("Heap initialization failed");
    framebuffer::init(&FRAMEBUFFER_REQUEST);
    console::clear_screen();

    serial_println!(" :: Butterscotch OS 0.1.0 Alpha :: ");
    serial_println!("Copyright 2024 Vardhan Patil");
    console_println!(" :: Butterscotch OS 0.1.0 Alpha :: ");
    console_println!("Copyright 2024 Vardhan Patil");
}
