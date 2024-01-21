use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::*;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

/// # Safety
/// 
/// Should only be called on one thread, once
/// Is called by main during startup
pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("CPU Exception: Breakpoint\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}