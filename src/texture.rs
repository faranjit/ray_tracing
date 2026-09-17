use crate::{color::Color, rtw_image::RtwImage, vec3::Point3};

#[derive(Clone)]
pub enum Texture {
    SolidColor(SolidColor),
    Checker(Checker),
    Image(ImageTexture),
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        match self {
            Texture::SolidColor(solid) => solid.value(u, v, p),
            Texture::Checker(checker) => checker.value(u, v, p),
            Texture::Image(image) => image.value(u, v, p),
        }
    }

    pub fn solid_color(color: Color) -> Self {
        Texture::SolidColor(SolidColor::new(color))
    }

    pub fn checker(scale: f64, even: Texture, odd: Texture) -> Self {
        Texture::Checker(Checker::new(scale, even, odd))
    }

    pub fn image(filename: &str) -> Self {
        Texture::Image(ImageTexture::new(filename))
    }
}

#[derive(Clone, Copy)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }

    pub fn rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }

    pub fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.albedo
    }
}

#[derive(Clone)]
pub struct Checker {
    inv_scale: f64,
    even: Box<Texture>,
    odd: Box<Texture>,
}

impl Checker {
    pub fn new(scale: f64, even: Texture, odd: Texture) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Box::new(even),
            odd: Box::new(odd),
        }
    }

    pub fn from_colors(scale: f64, color1: Color, color2: Color) -> Self {
        Checker::new(
            scale,
            Texture::SolidColor(SolidColor::new(color1)),
            Texture::SolidColor(SolidColor::new(color2)),
        )
    }

    pub fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_int = (self.inv_scale * p.x()).floor() as i64;
        let y_int = (self.inv_scale * p.y()).floor() as i64;
        let z_int = (self.inv_scale * p.z()).floor() as i64;

        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::new(filename),
        }
    }

    pub fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);  // Flip V to image coordinates

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;

        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
