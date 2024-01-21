use crate::*;

pub fn init() {
    interrupt::init_idt();
    println!("Butterscotch OS 0.1.0 Alpha");
}


