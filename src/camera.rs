use crate::{
    color::{BLACK, Color, write_color}, hittable::Hittable, interval::Interval, ray::Ray, rtweekend, vec3::{Point3, Vec3, unit_vector},
};

pub struct CameraConfig {
    image_width: u64,         // Rendered image width in pixel count
    aspect_ratio: f64,        // Ratio of image width over height
    samples_per_pixel: u16,   // Count of random samples for each pixel
    pixel_samples_scale: f64, // Scale factor to convert pixel samples to color intensity
    max_depth: u16,           // Maximum number of ray bounces into scene
}

impl CameraConfig {
    pub fn new(image_width: u64, aspect_ratio: f64, samples_per_pixel: u16, max_depth: u16) -> Self {
        Self {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / (samples_per_pixel as f64),
            max_depth: max_depth,
        }
    }
}

pub struct Camera {
    config: CameraConfig,
    image_height: u64,   // rendered image height
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
    pub fn initialize(config: CameraConfig) -> Self {
        let mut image_height = (config.image_width as f64 / config.aspect_ratio) as u64;
        if image_height < 1 {
            image_height = 1;
        }

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (config.image_width as f64 / image_height as f64);
        let center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / config.image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            config,
            image_height: image_height,
            center: center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.config.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.config.image_width {
                let mut pixel_color = BLACK;
                for _ in 0..self.config.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.config.max_depth, world);
                }

                write_color(pixel_color * self.config.pixel_samples_scale);
            }
        }
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = Vec3::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(&self, ray: &Ray, depth: u16, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return BLACK;
        }

        if let Some(rec) = world.hit(ray, Interval::new(0.001, rtweekend::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(&ray, &rec) {
                return attenuation * self.ray_color(&scattered, depth-1, world);
            }

            return BLACK;
            // let direction = rec.normal + random_unit_vector();
            // return 0.1 * self.ray_color(&Ray::new(rec.p, direction), depth - 1, world);
        }

        let unit_direction = unit_vector(ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }
}
