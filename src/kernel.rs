use crate::*;

pub fn init() {
    unsafe {
        gdt::init();
        interrupt::init();
    }
    println!(" :: Butterscotch OS 0.1.0 Alpha :: ");
    serial_println!(" :: Butterscotch OS 0.1.0 Alpha :: ");

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

}
