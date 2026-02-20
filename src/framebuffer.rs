use core::fmt;
use spin::Mutex;
use crate::font;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
    pub const CYAN: Color = Color { r: 0, g: 255, b: 255 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };

    pub fn to_argb(self) -> u32 {
        (0xFF << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

pub struct Framebuffer {
    address: *mut u32,
    width: usize,
    height: usize,
    pitch: usize,
    cursor_x: usize,
    cursor_y: usize,
    color: Color,
    bg_color: Color,
}

// Implementar Send y Sync manualmente porque son punteros crudos
unsafe impl Send for Framebuffer {}
unsafe impl Sync for Framebuffer {}

impl Framebuffer {
    pub unsafe fn new_from_limine(fb_info: &limine::response::FramebufferResponse) -> Option<Self> {
        let fb = fb_info.framebuffers().next()?;

        Some(Framebuffer {
            address: fb.addr() as *mut u32,
            width: fb.width() as usize,
            height: fb.height() as usize,
            pitch: fb.pitch() as usize,
            cursor_x: 0,
            cursor_y: 0,
            color: Color::WHITE,
            bg_color: Color::BLACK,
        })
    }

    pub unsafe fn clear(&mut self) {
        let color = self.bg_color.to_argb();
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = y * (self.pitch / 4) + x;
                *self.address.add(offset) = color;
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    #[inline]
    pub unsafe fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            let offset = y * (self.pitch / 4) + x;
            *self.address.add(offset) = color.to_argb();
        }
    }

    pub unsafe fn putchar(&mut self, c: u8) {
        if c == b'\n' {
            self.cursor_x = 0;
            self.cursor_y += 16;
            if self.cursor_y + 16 >= self.height {
                self.scroll();
            }
            return;
        }

        if c == b'\r' {
            self.cursor_x = 0;
            return;
        }

        if c == b'\t' {
            // Tabulador = 4 espacios
            for _ in 0..4 {
                self.putchar(b' ');
            }
            return;
        }

        // Caracteres no imprimibles se ignoran
        if c < 32 || c > 126 {
            return;
        }

        let char_data = font::get_char(c);

        for row in 0..16 {
            let byte = char_data[row];
            for col in 0..8 {
                if (byte >> (7 - col)) & 1 == 1 {
                    self.draw_pixel(
                        self.cursor_x + col,
                        self.cursor_y + row,
                        self.color
                    );
                } else {
                    self.draw_pixel(
                        self.cursor_x + col,
                        self.cursor_y + row,
                        self.bg_color
                    );
                }
            }
        }

        self.cursor_x += 8;
        if self.cursor_x + 8 >= self.width {
            self.cursor_x = 0;
            self.cursor_y += 16;
            if self.cursor_y + 16 >= self.height {
                self.scroll();
            }
        }
    }

    pub unsafe fn print(&mut self, s: &str) {
        for &byte in s.as_bytes() {
            self.putchar(byte);
        }
    }

    pub unsafe fn println(&mut self, s: &str) {
        self.print(s);
        self.putchar(b'\n');
    }

    unsafe fn scroll(&mut self) {
        let row_size = self.pitch / 4;
        let src = self.address.add(16 * row_size);
        let dst = self.address;

        for y in 0..(self.height - 16) {
            for x in 0..self.width {
                *dst.add(y * row_size + x) = *src.add(y * row_size + x);
            }
        }

        for y in (self.height - 16)..self.height {
            for x in 0..self.width {
                *dst.add(y * row_size + x) = self.bg_color.to_argb();
            }
        }

        self.cursor_y = self.height - 16;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_cursor_char(&mut self, col: usize, row: usize) {
        self.cursor_x = col * 8;
        self.cursor_y = row * 16;
    }
}

// Implementar Write para usar con print!
impl fmt::Write for Framebuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            self.print(s);
        }
        Ok(())
    }
}

// Writer global
pub static WRITER: Mutex<Option<Framebuffer>> = Mutex::new(None);
pub const INPUT_PROMPT: &str = "> ";

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = WRITER.lock();
    if let Some(fb) = writer.as_mut() {
        fb.write_fmt(args).unwrap();
    }
}
