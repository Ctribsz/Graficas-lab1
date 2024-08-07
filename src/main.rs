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

    //Poligono 1
    let points1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    //Poligono 2
    let points2 = [
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    // Rellenar el primer polígono con color amarillo
    fill_polygon(&mut fb, &points1, Color::YELLOW);

    // Dibujar la orilla del primer polígono con color blanco
    for i in 0..points1.len() {
        let (x0, y0) = points1[i];
        let (x1, y1) = points1[(i + 1) % points1.len()];
        draw_line(&mut fb, x0, y0, x1, y1, Color::WHITE);
    }

    // Rellenar el segundo polígono con color azul
    fill_polygon(&mut fb, &points2, Color::BLUE);

    // Dibujar la orilla del segundo polígono con color blanco
    for i in 0..points2.len() {
        let (x0, y0) = points2[i];
        let (x1, y1) = points2[(i + 1) % points2.len()];
        draw_line(&mut fb, x0, y0, x1, y1, Color::WHITE);
    }

    // Guardar el framebuffer como BMP
    BMP::save_as_bmp(&fb, "polygons1.bmp").unwrap();
}
