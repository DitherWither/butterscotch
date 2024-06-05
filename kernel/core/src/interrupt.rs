use crate::*;
use libk::sync::atomic::{AtomicU64, Ordering};
use libk::Mutex;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use pic8259::ChainedPics;
use x86_64::instructions::hlt;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

static KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
    ScancodeSet1::new(),
    layouts::Us104Key,
    HandleControl::Ignore,
));

static TIMER: AtomicU64 = AtomicU64::new(0);

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
    // FIXME keyboard inturrupts are not working at all
    IDT.double_fault.set_handler_fn(double_fault_handler);

    IDT[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
    IDT[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

    basic_handler!(IDT.breakpoint, "Breakpoint");
    IDT.page_fault.set_handler_fn(page_fault_handler);
    // TODO add handlers for other functions
    IDT.load();

    let mut pic_port: Port<u8> = Port::new(0x40);
    let rate = 1200u16; // 1ms
    unsafe {
        pic_port.write((rate & 0xFF) as u8);
        pic_port.write(((rate & 0xFF00) >> 8) as u8);
    }

    let mut mask = PICS.lock().read_masks();
    mask[0] &= &!(1 << 0);
    mask[0] &= &!(1 << 1);
    PICS.lock().write_masks(mask[0], mask[1]);
    PICS.lock().initialize();
    x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    eprintln!("CPU Exception: Page Fault");
    eprintln!("Accessed Address: {:?}", Cr2::read());
    eprintln!("Error Code: {:?}", error_code);
    eprintln!("{:#?}", stack_frame);
    hlt_loop();
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
    Keyboard,
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
    // TODO add a proper timer
    if TIMER.load(Ordering::Relaxed) > 0 {
        TIMER.fetch_sub(1, Ordering::Relaxed);
    }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

/// Wait for the amount of time in `time_millis`, and then return
///
/// Interrupts must be enabled, and initialized for this to work.
/// They should be initialized in kernel::init
pub fn sleep(time_millis: u64) {
    TIMER.store(time_millis, Ordering::Relaxed);

    while TIMER.load(Ordering::Relaxed) > 0 {
        hlt()
    }
}

/// Handler for the Keyboard events
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let mut keyboard = KEYBOARD.lock();

    // TODO add proper keyboard input
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::RawKey(_) => (),
                DecodedKey::Unicode(c) => {
                    // TODO Input might be dropped if it isn't consumed before the next keypress
                    *libk::io::stdin::CUR_CHAR.lock() = Some(c);
                }
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
