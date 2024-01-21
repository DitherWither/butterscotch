use crate::*;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

macro_rules! basic_handler {
    ($e:expr, $t:literal) => {{
        extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame) {
            $crate::eprintln!("CPU Exception: {}\n{:#?}", $t, stack_frame);
        }
        $e.set_handler_fn(handler);
    }};
}

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

    IDT[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);

    basic_handler!(IDT.breakpoint, "Breakpoint");
    // TODO add handlers for other functions

    IDT.load();

    PICS.lock().initialize();
    x86_64::instructions::interrupts::enable();
}


extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("CPU Exception: Double Fault\n{:#?}", stack_frame);
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

/// Handler for the PIT timer
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

// Testing double fault handler
extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
}
