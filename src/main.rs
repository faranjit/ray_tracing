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
    camera::{Camera, CameraConfig}, color::Color, hittable::{HittableList, Sphere}, material::{Lambertian, Metal}, vec3::{Point3, Vec3},
};

fn main() {
    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));

    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let camera = Camera::initialize(CameraConfig::new(512, 16.0 / 9.0, 100, 50));
    camera.render(&world);
}
