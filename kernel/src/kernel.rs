use crate::{
    io::{
        console,
        framebuffer,
    },
    *,
};

// Constants
const BASE_REVISION: u32 = 1;
const KERNEL_VERSION: &'static str = "0.1.0 Alpha";

pub static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);
static MEMMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(1);
static HHDM_REQUEST: limine::HhdmRequest = limine::HhdmRequest::new(1);

/// Initializes the operating system components, including interrupt handling,
/// memory management, framebuffer, and console.
pub fn init() -> Result<(), &'static str> {
    unsafe {
        interrupt::init();
        memory::init(&MEMMAP_REQUEST, &HHDM_REQUEST)?;
    }

    kernel_allocator::init()?;

    framebuffer::init(&FRAMEBUFFER_REQUEST);
    console::clear_screen();

    serial_println!(" :: Butterscotch OS {} :: ", KERNEL_VERSION);
    serial_println!("Copyright 2024 Vardhan Patil");
    console_println!(" :: Butterscotch OS {} :: ", KERNEL_VERSION);
    console_println!("Copyright 2024 Vardhan Patil");

    Ok(())
}
