use crate::framebuffer::FrameBuffer;
use crate::color::Color;

pub fn draw_line(fb: &mut FrameBuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        fb.set_pixel(x0 as usize, y0 as usize, color);

        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn fill_polygon(fb: &mut FrameBuffer, points: &[(i32, i32)], color: Color) {
    let mut nodes = Vec::new();
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;

    // Encuentra el mínimo y el máximo de Y
    for point in points {
        if point.1 < min_y {
            min_y = point.1;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    // Escanear línea por línea desde min_y hasta max_y
    for y in min_y..=max_y {
        nodes.clear();

        // Construir una lista de nodos
        let mut j = points.len() - 1;
        for i in 0..points.len() {
            if (points[i].1 < y && points[j].1 >= y) || (points[j].1 < y && points[i].1 >= y) {
                let x = points[i].0 + (y - points[i].1) * (points[j].0 - points[i].0) / (points[j].1 - points[i].1);
                nodes.push(x);
            }
            j = i;
        }

        // Ordenar nodos
        nodes.sort();

        // Rellenar entre pares de nodos
        for n in (0..nodes.len()).step_by(2) {
            if n + 1 < nodes.len() {
                for x in nodes[n]..=nodes[n + 1] {
                    fb.set_pixel(x as usize, y as usize, color);
                }
            }
        }
    }
}

pub fn point_in_polygon(point: (i32, i32), polygon: &[(i32, i32)]) -> bool {
    let (x, y) = point;
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

pub fn fill_polygon_with_hole(fb: &mut FrameBuffer, outer: &[(i32, i32)], hole: &[(i32, i32)], color: Color) {
    let mut nodes = Vec::new();
    let mut min_y = outer[0].1;
    let mut max_y = outer[0].1;

    // Encuentra el mínimo y el máximo de Y
    for point in outer {
        if point.1 < min_y {
            min_y = point.1;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    // Escanear línea por línea desde min_y hasta max_y
    for y in min_y..=max_y {
        nodes.clear();

        // Construir una lista de nodos
        let mut j = outer.len() - 1;
        for i in 0..outer.len() {
            if (outer[i].1 < y && outer[j].1 >= y) || (outer[j].1 < y && outer[i].1 >= y) {
                let x = outer[i].0 + (y - outer[i].1) * (outer[j].0 - outer[i].0) / (outer[j].1 - outer[i].1);
                nodes.push(x);
            }
            j = i;
        }

        // Ordenar nodos
        nodes.sort();

        // Rellenar entre pares de nodos
        for n in (0..nodes.len()).step_by(2) {
            if n + 1 < nodes.len() {
                for x in nodes[n]..=nodes[n + 1] {
                    if !point_in_polygon((x, y), hole) {
                        fb.set_pixel(x as usize, y as usize, color);
                    }
                }
            }
        }
    }
}
