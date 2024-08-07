mod framebuffer;
mod geometry;
mod color;
mod bmp;

use framebuffer::FrameBuffer;
use geometry::{draw_line, fill_polygon};
use color::Color;
use bmp::BMP;

fn main() {
    let mut fb = FrameBuffer::new(800, 600);

    let points1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    // Rellenar el primer polígono con color amarillo
    fill_polygon(&mut fb, &points1, Color::YELLOW);

    // Dibujar la orilla del primer polígono con color blanco
    for i in 0..points1.len() {
        let (x0, y0) = points1[i];
        let (x1, y1) = points1[(i + 1) % points1.len()];
        draw_line(&mut fb, x0, y0, x1, y1, Color::WHITE);
    }

    // Guardar el framebuffer como BMP
    BMP::save_as_bmp(&fb, "polygon_1.bmp").unwrap();
}
