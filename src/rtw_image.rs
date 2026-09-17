use image::{DynamicImage, GenericImageView};
use std::path::Path;

#[derive(Clone)]
pub struct RtwImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl RtwImage {
    pub fn new<P: AsRef<Path>>(filename: P) -> Self {
        match image::open(filename) {
            Ok(img) => {
                let (width, height) = img.dimensions();
                let data = img.into_rgb8().into_raw();
                Self {
                    data,
                    width,
                    height,
                }
            }
            Err(_) => {
                eprintln!("ERROR: Could not load image file.");
                Self {
                    data: Vec::new(),
                    width: 0,
                    height: 0,
                }
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> [u8; 3] {
        if self.data.is_empty() {
            return [255, 0, 255];
        }

        let x = x.clamp(0, self.width - 1);
        let y = y.clamp(0, self.height - 1);

        let index = ((y * self.width + x) * 3) as usize;

        [self.data[index], self.data[index + 1], self.data[index + 2]]
    }
}
