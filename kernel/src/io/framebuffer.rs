use limine::{Framebuffer, NonNullPtr};
use spin::Mutex;

pub static FRAMEBUFFER: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);

pub fn init(frame_buffer_request: &limine::FramebufferRequest) {
    let mut fb = FRAMEBUFFER.lock();
    *fb = Some(FrameBufferWriter::new(frame_buffer_request))
}

pub fn set_pixel(x: usize, y: usize, color: u32) {
    if let Some(fb) = &mut *FRAMEBUFFER.lock() {
        fb.set_pixel(x, y, color);
    }
}

pub fn fill_rect(x: usize, y: usize, width: usize, height: usize, color: u32) {
    if let Some(fb) = &mut *FRAMEBUFFER.lock() {
        fb.fill_rect(x, y, width, height, color);
    }
}

pub fn clear(color: u32) {
    if let Some(fb) = &mut *FRAMEBUFFER.lock() {
        fb.clear(color);
    }
}

pub fn width() -> usize {
    if let Some(fb) = &mut *FRAMEBUFFER.lock() {
        fb.framebuffer.width as usize
    } else {
        0
    }
}

pub fn height() -> usize {
    if let Some(fb) = &mut *FRAMEBUFFER.lock() {
        fb.framebuffer.height as usize
    } else {
        0
    }
}

pub struct FrameBufferWriter {
    framebuffer: &'static NonNullPtr<Framebuffer>,
}

impl FrameBufferWriter {
    pub fn new(frame_buffer_request: &limine::FramebufferRequest) -> Self {
        let fb_response = frame_buffer_request.get_response().get().unwrap();
        if fb_response.framebuffer_count < 1 {
            panic!("Less than 1 framebuffers found");
        }

        let framebuffer = &fb_response.framebuffers()[0];

        Self { framebuffer }
    }

    fn set_pixel(&self, x: usize, y: usize, color: u32) {
        let offset = self.framebuffer.pitch as usize * y + x * 4;

        unsafe {
            *(self
                .framebuffer
                .address
                .as_ptr()
                .unwrap()
                .add(offset) as *mut u32) = color;
        }
    }

    fn fill_rect(&self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let buf = self.framebuffer.address.as_ptr().unwrap();
        for i in y..(y + height) {
            let y_offset = i * self.framebuffer.pitch as usize;
            let buf = unsafe { buf.add(y_offset) };
            for j in x..(x + width) {
                unsafe { *(buf.add(j * 4) as *mut u32) = color };
            }
        }
    }

    fn clear(&self, color: u32) {
        self.fill_rect(
            0,
            0,
            self.framebuffer.width as usize,
            self.framebuffer.height as usize,
            color,
        );
    }
}

pub mod color {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }
    pub fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> u32 {
        // This code is horrendous to read, also ICantBelieveItsNotLisp(TM)
        ((((r as f32) * a) as u32) << 16)
            | ((((g as f32) * a) as u32) << 8)
            | (((b as f32) * a) as u32)
    }
}
