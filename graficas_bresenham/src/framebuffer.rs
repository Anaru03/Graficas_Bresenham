use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = unsafe {
            Image::from_raw(raylib::ffi::GenImageColor(
                width as i32,
                height as i32,
                Color::BLACK.into(),
            ))
        };

        Framebuffer {
            width,
            height,
            color_buffer,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = unsafe {
            Image::from_raw(raylib::ffi::GenImageColor(
                self.width as i32,
                self.height as i32,
                self.background_color.into(),
            ))
        };
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.color_buffer
            .draw_pixel(x as i32, y as i32, self.current_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&mut self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}