use spin::Mutex;
use uart_16550::SerialPort;

pub static SERIAL1: Mutex<Serial> = Mutex::new(Serial::new(0x3F8));

#[derive(Debug)]
pub struct Serial {
    port: SerialPort,
    is_initialized: bool,
}

impl Serial {
    const fn new(base: u16) -> Self {
        Self {
            port: unsafe { SerialPort::new(base) },
            is_initialized: false,
        }
    }
}

#[doc(hidden)]
pub fn _serial_print(args: ::core::fmt::Arguments) {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut serial = SERIAL1.lock();
        if !serial.is_initialized {
            serial.port.init();
        }
        use core::fmt::Write;
        let _ = serial
            .port
            .write_fmt(args);
    })
}
