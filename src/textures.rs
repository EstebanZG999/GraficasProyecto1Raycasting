extern crate image;

use image::{ImageReader, GenericImageView};

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub color_array: Vec<Vec<u32>>,
}

impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = img.width();
        let height = img.height();
        let mut color_array = vec![vec![0; height as usize]; width as usize];

        for x in 0..width {
            for y in 0..height {
                let pixel = img.get_pixel(x, y).0;
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                color_array[x as usize][y as usize] = color;
            }
        }

        Texture { width, height, color_array }
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> u32 {
        self.color_array[x as usize % self.width as usize][y as usize % self.height as usize]
    }
}
