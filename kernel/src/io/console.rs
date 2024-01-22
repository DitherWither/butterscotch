use core::fmt;
use core::fmt::Write;
use noto_sans_mono_bitmap::get_raster;
use noto_sans_mono_bitmap::get_raster_width;
use noto_sans_mono_bitmap::FontWeight;
use noto_sans_mono_bitmap::RasterHeight;
use noto_sans_mono_bitmap::RasterizedChar;
use spin::Mutex;

use super::framebuffer;
use super::framebuffer::color;

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;
const DEFAULT_BACKGROUND_COLOR: u32 = color::from_rgb(20, 20, 20);

mod font_constants {

    use super::*;

    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
    pub const BACKUP_CHAR: char = '�'; // Fallback if character can't be printed
    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
}

/// Gets the raster of the character, or backup character
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(
            c,
            font_constants::FONT_WEIGHT,
            font_constants::CHAR_RASTER_HEIGHT,
        )
    }
    get(c).unwrap_or_else(|| {
        get(font_constants::BACKUP_CHAR).expect("Should get raster of backup char.")
    })
}

pub static CONSOLE: Mutex<Console> = Mutex::new(Console::new());

pub struct Console {
    x_pos: usize,
    y_pos: usize,
    text_color_r: u8,
    text_color_g: u8,
    text_color_b: u8,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            x_pos: BORDER_PADDING,
            y_pos: BORDER_PADDING,
            text_color_r: 255,
            text_color_b: 255,
            text_color_g: 255,
        }
    }

    pub fn clear_screen(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        framebuffer::clear(DEFAULT_BACKGROUND_COLOR);
    }

    pub fn newline(&mut self) {
        self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return();
    }

    pub fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    pub fn write_char(&mut self, c: char) {
        if framebuffer::width() == 0 || framebuffer::height() == 0 {
            return;
        }
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= framebuffer::width() {
                    self.newline();
                }
                let new_ypos =
                    self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= framebuffer::height() {
                    self.clear_screen(); // TODO implement scrolling
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&self, x: usize, y: usize, intensity: u8) {
        let intensity_norm = intensity as f32 / u8::MAX as f32;
        let color = color::from_rgba(20, 20, 20, 1.0 - intensity_norm)
            + color::from_rgba(
                self.text_color_r,
                self.text_color_g,
                self.text_color_b,
                intensity_norm,
            );

        framebuffer::set_pixel(x, y, color);
    }
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

pub fn clear_screen() {
    CONSOLE.lock().clear_screen();
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut con = CONSOLE.lock();
    let _ = con.write_fmt(args);
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    _print(args);
    crate::io::serial::_serial_print(args);
}
