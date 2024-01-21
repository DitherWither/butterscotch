use crate::*;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

/// # Safety
///
/// Should only be called on one thread, once
/// Is called by main during startup
pub unsafe fn init() {
    _init(false)
}

/// Same as init, but provides a option to not panic, and instead
/// exit with a success code
///
/// This is used in testing
///
/// # Safety
///
/// Should only be called on one thread, once
/// Is called by main during startup
pub unsafe fn _init(test_handler: bool) {
    IDT.double_fault
        .set_handler_fn(if test_handler {
            test_double_fault_handler
        } else {
            double_fault_handler
        })
        .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("CPU Exception: Double Fault\n{:#?}", stack_frame);
}

// Testing double fault handler
extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("CPU Exception: Breakpoint\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
