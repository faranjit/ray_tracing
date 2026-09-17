mod camera;
mod color;
mod hittable;
mod interval;
mod material;
pub mod ray;
mod rtweekend;
pub mod vec3;

use crate::{
    camera::{Camera, CameraConfig},
    color::Color,
    hittable::{HittableList, Object},
    material::Material,
    rtweekend::{random_double, random_double_range},
    vec3::{Point3, Vec3},
};

fn main() {
    let mut world = HittableList::new();
    world.add(Object::sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian(Color::new(0.5, 0.5, 0.5)),
    ));

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
                    world.add(Object::sphere(center, 0.2, Material::lambertian(albedo)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    world.add(Object::sphere(center, 0.2, Material::metal(albedo, fuzz)));
                } else {
                    // glass
                    world.add(Object::sphere(center, 0.2, Material::dielectric(1.5)));
                }
            }
        }
    }

    world.add(Object::sphere(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Material::dielectric(1.5),
    ));

    world.add(Object::sphere(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::lambertian(Color::new(0.4, 0.2, 0.1)),
    ));

    world.add(Object::sphere(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Material::metal(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

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
