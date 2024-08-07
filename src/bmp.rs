use std::fs::File;
use std::io::{self, Write};
use crate::framebuffer::FrameBuffer;

pub struct BMP;

impl BMP {
    pub fn save_as_bmp(fb: &FrameBuffer, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        
        let file_header = create_file_header(fb.width, fb.height);
        let info_header = create_info_header(fb.width, fb.height);

        file.write_all(&file_header)?;
        file.write_all(&info_header)?;

        let padding = (4 - (fb.width * 3 % 4)) % 4;
        let mut buffer = Vec::with_capacity((fb.width * 3 + padding) * fb.height);

        for y in (0..fb.height).rev() {
            for x in 0..fb.width {
                let pixel = fb.get_pixel(x, y);
                buffer.extend_from_slice(&[pixel.b, pixel.g, pixel.r]);
            }
            buffer.extend(vec![0; padding]);
        }

        file.write_all(&buffer)?;
        Ok(())
    }
}

fn create_file_header(width: usize, height: usize) -> [u8; 14] {
    let file_size = 14 + 40 + (width * height * 3) as u32 + (height as u32) * ((4 - (width * 3 % 4)) % 4) as u32;
    [
        0x42, 0x4D, // Signature 'BM'
        (file_size & 0xFF) as u8, ((file_size >> 8) & 0xFF) as u8, ((file_size >> 16) & 0xFF) as u8, ((file_size >> 24) & 0xFF) as u8, // File size
        0x00, 0x00, // Reserved
        0x00, 0x00, // Reserved
        0x36, 0x00, 0x00, 0x00, // Offset to pixel data
    ]
}

fn create_info_header(width: usize, height: usize) -> [u8; 40] {
    [
        0x28, 0x00, 0x00, 0x00, // Header size
        (width as u32 & 0xFF) as u8, ((width as u32 >> 8) & 0xFF) as u8, ((width as u32 >> 16) & 0xFF) as u8, ((width as u32 >> 24) & 0xFF) as u8, // Width
        (height as u32 & 0xFF) as u8, ((height as u32 >> 8) & 0xFF) as u8, ((height as u32 >> 16) & 0xFF) as u8, ((height as u32 >> 24) & 0xFF) as u8, // Height
        0x01, 0x00, // Planes
        0x18, 0x00, // Bits per pixel (24)
        0x00, 0x00, 0x00, 0x00, // Compression (none)
        0x00, 0x00, 0x00, 0x00, // Image size (can be 0 for no compression)
        0x13, 0x0B, 0x00, 0x00, // X pixels per meter (2835)
        0x13, 0x0B, 0x00, 0x00, // Y pixels per meter (2835)
        0x00, 0x00, 0x00, 0x00, // Total colors (0 = 2^n)
        0x00, 0x00, 0x00, 0x00, // Important colors (0 = all)
    ]
}
