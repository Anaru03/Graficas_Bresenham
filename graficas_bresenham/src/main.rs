mod bmp;
mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::draw_line;
use raylib::prelude::*;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    framebuffer.set_current_color(Color::WHITE);

    draw_line(
        &mut framebuffer,
        50,
        50,
        350,
        350,
    );

    framebuffer.render_to_file("out.bmp");
}