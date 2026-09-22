use std::{
    io::{self, BufWriter, Write},
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant},
};

use crate::{
    color::{BLACK, Color, write_color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    rtweekend::{INFINITY, Real, degrees_to_radians, random_double},
    vec3::{Point3, Vec3, random_in_unit_disk, unit_vector},
};
use rayon::prelude::*;

pub struct CameraConfig {
    image_width: u64,          // Rendered image width in pixel count
    aspect_ratio: Real,        // Ratio of image width over height
    samples_per_pixel: u32,    // Count of random samples for each pixel
    pixel_samples_scale: Real, // Scale factor to convert pixel samples to color intensity
    max_depth: u16,            // Maximum number of ray bounces into scene
    vfov: Real,
    look_from: Point3,   // Point camera is looking from
    look_at: Point3,     // Point camera is looking at
    vup: Vec3,           // Camera-relative "up" direction
    defocus_angle: Real, // Variation angle of rays through each pixel
    focus_dist: Real,    // Distance from camera lookfrom point to plane of perfect focus
    background: Color,   // Scene background color
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
            background: BLACK,
        }
    }
}

impl CameraConfig {
    pub fn image_width(mut self, width: u64) -> Self {
        self.image_width = width;
        self
    }

    pub fn aspect_ratio(mut self, ratio: Real) -> Self {
        self.aspect_ratio = ratio;
        self
    }

    pub fn samples_per_pixel(mut self, samples: u32) -> Self {
        self.samples_per_pixel = samples;
        self.pixel_samples_scale = 1.0 / (samples as Real);
        self
    }

    pub fn max_depth(mut self, depth: u16) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn vfov(mut self, vfov: Real) -> Self {
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

    pub fn defocus_angle(mut self, angle: Real) -> Self {
        self.defocus_angle = angle;
        self
    }

    pub fn focus_dist(mut self, dist: Real) -> Self {
        self.focus_dist = dist;
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }
}

pub struct Camera {
    config: CameraConfig,
    image_height: u64,   // Rendered image height
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn initialize(config: CameraConfig) -> Self {
        let image_height = ((config.image_width as Real / config.aspect_ratio) as u64).max(1);

        let center = config.look_from;
        let look_direction = config.look_from - config.look_at;

        let theta = degrees_to_radians(config.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * config.focus_dist;
        let viewport_width = viewport_height * (config.image_width as Real / image_height as Real);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = unit_vector(look_direction);
        let u = unit_vector(config.vup.cross(w));
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / config.image_width as Real;
        let pixel_delta_v = viewport_v / image_height as Real;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (config.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius =
            config.focus_dist * (degrees_to_radians(config.defocus_angle) / 2.0).tan();

        Self {
            config,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u: u * defocus_radius,
            defocus_disk_v: v * defocus_radius,
        }
    }

    pub fn render(&self, world: &(impl Hittable + Sync)) {
        let start = Instant::now();
        let total_pixels = (self.image_height * self.config.image_width) as usize;
        let width = self.config.image_width as usize;
        let done = AtomicU64::new(0);

        // The progress thread reads the counter once a second. The render threads
        // only touch an atomic, so they never wait on stderr.
        let pixels: Vec<Color> = thread::scope(|s| {
            s.spawn(|| {
                loop {
                    let n = done.load(Ordering::Relaxed);
                    eprint!("\rprogress: {n}/{total_pixels}");
                    if n >= total_pixels as u64 {
                        break;
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            });

            (0..total_pixels)
                .into_par_iter()
                .with_min_len(256)
                .map(|idx| {
                    let j = (idx / width) as u64;
                    let i = (idx % width) as u64;

                    let mut pixel_color = BLACK;
                    for _ in 0..self.config.samples_per_pixel {
                        let ray = self.get_ray(i, j);
                        pixel_color += self.ray_color(&ray, world);
                    }

                    done.fetch_add(1, Ordering::Relaxed);
                    pixel_color * self.config.pixel_samples_scale
                })
                .collect()
        });

        eprintln!("\rrendered in {:.1?}{:20}", start.elapsed(), "");

        // stdout is locked only after every thread is done.
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        writeln!(
            out,
            "P3\n{} {}\n255",
            self.config.image_width, self.image_height
        )
        .unwrap();

        for pixel_color in pixels {
            write_color(&mut out, pixel_color).unwrap();
        }
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = Vec3::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as Real + offset.x()) * self.pixel_delta_u)
            + ((j as Real + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.config.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        Ray::new_at_time(ray_origin, pixel_sample - ray_origin, random_double())
    }

    fn ray_color<H: Hittable>(&self, ray: &Ray, world: &H) -> Color {
        let mut ray = *ray;
        let mut attenuation = Color::new(1.0, 1.0, 1.0);
        let mut emitted_total = BLACK;

        for _ in 0..self.config.max_depth {
            let Some(rec) = world.hit(&ray, Interval::new(0.001, INFINITY)) else {
                return emitted_total + attenuation * self.config.background;
            };

            emitted_total += attenuation * rec.mat.emitted(rec.u, rec.v, rec.p);

            let Some((att, scattered)) = rec.mat.scatter(&ray, &rec) else {
                return emitted_total;
            };

            attenuation = attenuation * att;
            ray = scattered;

            // Russian-roulette style cutoff: once the path can no longer
            // contribute a visible amount, stop tracing it.
            if attenuation.len_squared() < 1e-8 {
                return emitted_total;
            }
        }

        emitted_total
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}
