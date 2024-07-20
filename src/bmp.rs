use std::fs::File;
use std::io::{Write, Result as IoResult};

pub struct BMPHeader {
    pub file_size: u32,
    pub reserved: u16,
    pub offset: u32,
    pub header_size: u32,
    pub width: u32,
    pub height: u32,
    pub planes: u16,
    pub bits_per_pixel: u16,
    pub compression: u32,
    pub image_size: u32,
    pub x_resolution: u32,
    pub y_resolution: u32,
    pub colors_used: u32,
    pub important_colors: u32,
}

impl BMPHeader {
    pub fn new(width: u32, height: u32) -> Self {
        let file_size = 54 + (width * height * 3); // Assuming 24 bits per pixel
        Self {
            file_size,
            reserved: 0,
            offset: 54,
            header_size: 40,
            width,
            height,
            planes: 1,
            bits_per_pixel: 24,
            compression: 0,
            image_size: width * height * 3,
            x_resolution: 0,
            y_resolution: 0,
            colors_used: 0,
            important_colors: 0,
        }
    }

    pub fn write_to_file(&self, file: &mut File) -> IoResult<()> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&[
            0x42, 0x4D, // BMP Signature
            0, 0, 0, 0, // File size (to be filled later)
            0, 0, // Reserved
            0, 0, // Reserved
            54, 0, 0, 0, // Offset to pixel data
            40, 0, 0, 0, // Header size
            self.width as u8, (self.width >> 8) as u8, (self.width >> 16) as u8, (self.width >> 24) as u8, // Width
            self.height as u8, (self.height >> 8) as u8, (self.height >> 16) as u8, (self.height >> 24) as u8, // Height
            1, 0, // Planes
            self.bits_per_pixel as u8, (self.bits_per_pixel >> 8) as u8, // Bits per pixel
            0, 0, 0, 0, // Compression
            self.image_size as u8, (self.image_size >> 8) as u8, (self.image_size >> 16) as u8, (self.image_size >> 24) as u8, // Image size
            0, 0, 0, 0, // X resolution
            0, 0, 0, 0, // Y resolution
            0, 0, 0, 0, // Colors used
            0, 0, 0, 0, // Important colors
        ]);

        let file_size = buffer.len() as u32;
        buffer[2..6].copy_from_slice(&file_size.to_le_bytes());

        file.write_all(&buffer)
    }
}
