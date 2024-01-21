use crate::*;

pub fn init() {
    unsafe {
        gdt::init();
        interrupt::init();
    }
    println!(" :: Butterscotch OS 0.1.0 Alpha :: ");
    serial_println!(" :: Butterscotch OS 0.1.0 Alpha :: ");
}
