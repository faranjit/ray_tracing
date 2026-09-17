mod camera;
mod color;
mod hittable;
mod interval;
mod material;
pub mod ray;
mod rtweekend;
pub mod vec3;

use std::sync::Arc;

use crate::{
    camera::{Camera, CameraConfig},
    color::Color,
    hittable::{HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    rtweekend::{random_double, random_double_range},
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Camera
    let config = CameraConfig::default()
        .image_width(1200)
        .aspect_ratio(16.0 / 9.0)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(20.0)
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.6)
        .focus_dist(10.0);
    let camera = Camera::initialize(config);
    camera.render(&world);
}
