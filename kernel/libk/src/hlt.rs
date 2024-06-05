use core::arch::asm;

// TODO explicitly check if platform is x86
pub fn hlt() {
    unsafe { asm!("hlt") }
}
