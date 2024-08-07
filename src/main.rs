mod framebuffer;
mod geometry;
mod color;
mod bmp;

use framebuffer::FrameBuffer;
use geometry::{draw_line, fill_polygon, fill_polygon_with_hole};
use color::Color;
use bmp::BMP;

fn main() {
    let mut fb = FrameBuffer::new(800, 600);

    let points1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    let points2 = [
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    let points3 = [
        (377, 249), (411, 197), (436, 249),
    ];

    let points4 = [
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36),
        (676, 37), (660, 52), (750, 145), (761, 179), (672, 192),
        (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    let points5 = [
        (682, 175), (708, 120), (735, 148), (739, 170),
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

    // Rellenar el tercer polígono con color rojo
    fill_polygon(&mut fb, &points3, Color::RED);

    // Dibujar la orilla del tercer polígono con color blanco
    for i in 0..points3.len() {
        let (x0, y0) = points3[i];
        let (x1, y1) = points3[(i + 1) % points3.len()];
        draw_line(&mut fb, x0, y0, x1, y1, Color::WHITE);
    }

    // Rellenar el cuarto polígono con color verde y un agujero del quinto polígono
    fill_polygon_with_hole(&mut fb, &points4, &points5, Color::GREEN);

    // Dibujar la orilla del cuarto polígono con color blanco
    for i in 0..points4.len() {
        let (x0, y0) = points4[i];
        let (x1, y1) = points4[(i + 1) % points4.len()];
        draw_line(&mut fb, x0, y0, x1, y1, Color::WHITE);
    }

    // Dibujar la orilla del agujero (quinto polígono) con color blanco
    for i in 0..points5.len() {
        let (x0, y0) = points5[i];
        let (x1, y1) = points5[(i + 1) % points5.len()];
        draw_line(&mut fb, x0, y0, x1, y1, Color::WHITE);
    }

    // Guardar el framebuffer como BMP
    BMP::save_as_bmp(&fb, "polygons3.bmp").unwrap();
}
