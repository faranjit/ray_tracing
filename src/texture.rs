use crate::{color::Color, perlin::Perlin, rtw_image::RtwImage, rtweekend::Real, vec3::Point3};

#[derive(Clone)]
pub enum Texture {
    SolidColor(SolidColor),
    Checker(Checker),
    Image(ImageTexture),
    Noise(NoiseTexture),
}

impl Texture {
    pub fn value(&self, u: Real, v: Real, p: Point3) -> Color {
        match self {
            Texture::SolidColor(solid) => solid.value(u, v, p),
            Texture::Checker(checker) => checker.value(u, v, p),
            Texture::Image(image) => image.value(u, v, p),
            Texture::Noise(noise) => noise.value(p),
        }
    }

    pub fn needs_uv(&self) -> bool {
        match self {
            Texture::Image(_) => true,
            Texture::Checker(c) => c.needs_uv(),
            _ => false,
        }
    }

    pub fn solid_color(color: Color) -> Self {
        Texture::SolidColor(SolidColor::new(color))
    }

    pub fn checker(scale: Real, even: Texture, odd: Texture) -> Self {
        Texture::Checker(Checker::new(scale, even, odd))
    }

    pub fn image(filename: &str) -> Self {
        Texture::Image(ImageTexture::new(filename))
    }

    pub fn noise(scale: Real) -> Self {
        Texture::Noise(NoiseTexture::new(scale))
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

    pub fn value(&self, _u: Real, _v: Real, _p: Point3) -> Color {
        self.albedo
    }
}

#[derive(Clone)]
pub struct Checker {
    inv_scale: Real,
    even: Box<Texture>,
    odd: Box<Texture>,
}

impl Checker {
    pub fn new(scale: Real, even: Texture, odd: Texture) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Box::new(even),
            odd: Box::new(odd),
        }
    }

    pub fn needs_uv(&self) -> bool {
        self.even.needs_uv() || self.odd.needs_uv()
    }

    pub fn value(&self, u: Real, v: Real, p: Point3) -> Color {
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

    pub fn value(&self, u: Real, v: Real, _p: Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let i = (u * self.image.width() as Real) as u32;
        let j = (v * self.image.height() as Real) as u32;

        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;

        Color::new(
            color_scale * Real::from(pixel[0]),
            color_scale * Real::from(pixel[1]),
            color_scale * Real::from(pixel[2]),
        )
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: Real,
}

impl NoiseTexture {
    pub fn new(scale: Real) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }

    pub fn value(&self, p: Point3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}
