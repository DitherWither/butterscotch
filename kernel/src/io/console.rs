use alloc::vec::Vec;
use core::fmt;
use core::fmt::Write;
use noto_sans_mono_bitmap::get_raster;
use noto_sans_mono_bitmap::get_raster_width;
use noto_sans_mono_bitmap::FontWeight;
use noto_sans_mono_bitmap::RasterHeight;
use noto_sans_mono_bitmap::RasterizedChar;
use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;

use self::font_constants::CHAR_RASTER_HEIGHT;
use self::font_constants::CHAR_RASTER_WIDTH;

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
    get_raster(
        c,
        font_constants::FONT_WEIGHT,
        font_constants::CHAR_RASTER_HEIGHT,
    )
    .unwrap_or_else(|| {
        get_raster(
            font_constants::BACKUP_CHAR,
            font_constants::FONT_WEIGHT,
            font_constants::CHAR_RASTER_HEIGHT,
        )
        .expect("Should get raster of backup char.")
    })
}

pub static CONSOLE: Mutex<Console> = Mutex::new(Console::new());

pub struct Console {
    rendered_chars: Option<Vec<Vec<Vec<u32>>>>,
    x_pos: usize,
    y_pos: usize,
    text_color_r: u8,
    text_color_g: u8,
    text_color_b: u8,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            rendered_chars: None,
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

        if self.rendered_chars.is_none() {
            self.rendered_chars = Some(self.render_chars());
        }
    }

    pub fn render_chars(&mut self) -> Vec<Vec<Vec<u32>>> {
        let mut rendered_chars = Vec::with_capacity(u8::MAX as usize);
        for i in 0..u8::MAX {
            let character = get_char_raster(i.into());

            let v: Vec<Vec<u32>> = (0..CHAR_RASTER_HEIGHT as usize)
                .map(|i| {
                    let r = character.raster()[i];
                    (0..CHAR_RASTER_WIDTH)
                        .map(|j| {
                            let intensity = r[j];
                            let intensity_norm = intensity as f32 / u8::MAX as f32;
                            color::from_rgba(20, 20, 20, 1.0 - intensity_norm)
                                + color::from_rgba(
                                    self.text_color_r,
                                    self.text_color_g,
                                    self.text_color_b,
                                    intensity_norm,
                                )
                        })
                        .collect::<Vec<u32>>()
                })
                .collect();
            rendered_chars.push(v)
        }
        rendered_chars
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
                // Draw the character by copying bytes from the prerendered buffer
                if let Some(chars) = &self.rendered_chars {
                    let char = &chars[c as usize];
                    for (i, line) in char.iter().enumerate() {
                        for (j, pixel) in line.iter().enumerate() {
                            framebuffer::set_pixel(self.x_pos + j, self.y_pos + i, *pixel)
                        }
                    }
                    self.x_pos += char[0].len() + LETTER_SPACING;
                }
            }
        }
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
    without_interrupts(|| {
        CONSOLE.lock().clear_screen();
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    without_interrupts(|| {
        let mut con = CONSOLE.lock();
        let _ = con.write_fmt(args);
    });
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    _print(args);
    crate::io::serial::_serial_print(args);
}
