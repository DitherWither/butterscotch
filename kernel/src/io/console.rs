use core::fmt;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub static CONSOLE: Mutex<Writer> = Mutex::new(Writer {
    x_position: 0,
    text_color: TextColor::new(Color::White, Color::Black),
    buffer: Buffer { addr: 0xb8000 },
});

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct TextColor(u8);

impl TextColor {
    const fn new(foreground_color: Color, background_color: Color) -> TextColor {
        TextColor((background_color as u8) << 4 | (foreground_color as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenCharacter {
    ascii_character: u8,
    text_color: TextColor,
}

#[repr(transparent)]
struct Buffer {
    // Storing the address as a pointer causes issues
    // with the compiler. Notably, it wants us to implement
    // `Send` on it. The way to fix this is to just store
    // the raw address of the pointer, and cast it at the
    // last moment
    addr: usize,
}

impl Buffer {
    fn write(&self, x: usize, y: usize, value: ScreenCharacter) {
        let ptr = self.addr as *mut ScreenCharacter;
        unsafe { ptr.add(y * BUFFER_WIDTH + x).write_volatile(value) }
    }

    fn get(&self, x: usize, y: usize) -> ScreenCharacter {
        let ptr = self.addr as *mut ScreenCharacter;
        unsafe { ptr.add(y * BUFFER_WIDTH + x).read_volatile() }
    }
}

pub struct Writer {
    x_position: usize,
    text_color: TextColor,
    buffer: Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
            return;
        }

        if self.x_position >= BUFFER_WIDTH {
            self.new_line();
        }
        self.buffer.write(
            self.x_position,
            BUFFER_HEIGHT - 1,
            ScreenCharacter {
                ascii_character: byte,
                text_color: self.text_color,
            },
        );
        self.x_position += 1;
    }

    pub fn backspace(&mut self) {
        if self.x_position == 0 {
            return;
        }
        self.x_position -= 1;
        self.write_byte(b' ');
        self.x_position -= 1;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                8 => self.backspace(),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for i in 1..BUFFER_HEIGHT {
            for j in 0..BUFFER_WIDTH {
                let character = self.buffer.get(j, i);
                self.buffer.write(j, i - 1, character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.x_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenCharacter {
            ascii_character: b' ',
            text_color: self.text_color,
        };

        for i in 0..BUFFER_WIDTH {
            self.buffer.write(i, row, blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| CONSOLE.lock().write_fmt(args).unwrap())
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    _print(args);
    #[cfg(not(test))]
    crate::io::serial::_serial_print(args);
}

#[test_case]
fn test_println_simple() {
    // If the implementation didn't panic, its safe to assume it succeeded
    crate::println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..256 {
        crate::println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    crate::println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = CONSOLE.lock().buffer.get(i, BUFFER_HEIGHT - 2);
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
