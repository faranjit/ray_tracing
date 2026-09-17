use crate::{
    color::{BLACK, Color, write_color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    rtweekend::{self, degrees_to_radians},
    vec3::{Point3, Vec3, random_in_unit_disk, unit_vector},
};

pub struct CameraConfig {
    image_width: u64,         // Rendered image width in pixel count
    aspect_ratio: f64,        // Ratio of image width over height
    samples_per_pixel: u16,   // Count of random samples for each pixel
    pixel_samples_scale: f64, // Scale factor to convert pixel samples to color intensity
    max_depth: u16,           // Maximum number of ray bounces into scene
    vfov: f64,
    look_from: Point3,  // Point camera is looking from
    look_at: Point3,    // Point camera is looking at
    vup: Vec3,          // Camera-relative "up" direction
    defocus_angle: f64, // Variation angle of rays through each pixel
    focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            image_width: 100,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 10,
            pixel_samples_scale: 0.1,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

impl CameraConfig {
    pub fn image_width(mut self, width: u64) -> Self {
        self.image_width = width;
        self
    }

    pub fn aspect_ratio(mut self, ratio: f64) -> Self {
        self.aspect_ratio = ratio;
        self
    }

    pub fn samples_per_pixel(mut self, samples: u16) -> Self {
        self.samples_per_pixel = samples;
        // scale değerini de burada otomatik güncelliyoruz
        self.pixel_samples_scale = 1.0 / (samples as f64);
        self
    }

    pub fn max_depth(mut self, depth: u16) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn look_from(mut self, look_from: Point3) -> Self {
        self.look_from = look_from;
        self
    }

    pub fn look_at(mut self, look_at: Point3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn defocus_angle(mut self, angle: f64) -> Self {
        self.defocus_angle = angle;
        self
    }

    pub fn focus_dist(mut self, dist: f64) -> Self {
        self.focus_dist = dist;
        self
    }
}

pub struct Camera {
    config: CameraConfig,
    image_height: u64,   // rendered image height
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn initialize(config: CameraConfig) -> Self {
        let mut image_height = (config.image_width as f64 / config.aspect_ratio) as u64;
        if image_height < 1 {
            image_height = 1;
        }

        let center = config.look_from;
        let look_direction = config.look_from - config.look_at;

        let theta = degrees_to_radians(config.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * config.focus_dist;
        let viewport_width = viewport_height * (config.image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = unit_vector(look_direction);
        let u = unit_vector(config.vup.cross(&w));
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / config.image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (config.focus_dist * w) - viewport_u / 2 - viewport_v / 2;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius =
            config.focus_dist * (degrees_to_radians(config.defocus_angle) / 2.0).tan();

        Self {
            config,
            image_height: image_height,
            center: center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            defocus_disk_u: u * defocus_radius,
            defocus_disk_v: v * defocus_radius,
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

        let ray_origin = if self.config.defocus_angle < 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(&self, ray: &Ray, depth: u16, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return BLACK;
        }

        if let Some(rec) = world.hit(ray, Interval::new(0.001, rtweekend::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(&ray, &rec) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }

            return BLACK;
            // let direction = rec.normal + random_unit_vector();
            // return 0.1 * self.ray_color(&Ray::new(rec.p, direction), depth - 1, world);
        }

        let unit_direction = unit_vector(ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}
