mod aabb;
mod bvh_node;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod rtw_image;
mod rtweekend;
mod texture;
mod vec3;

use crate::{
    bvh_node::BVHNode,
    camera::{Camera, CameraConfig},
    color::Color,
    hittable::{HittableList, Object},
    material::Material,
    rtweekend::{random_double, random_double_range},
    texture::Texture,
    vec3::{Point3, Vec3},
};

fn bouncing_spheres() {
    let mut world = HittableList::new();

    let checker = Texture::checker(
        0.32,
        Texture::solid_color(Color::new(0.2, 0.3, 0.1)),
        Texture::solid_color(Color::new(0.9, 0.9, 0.9)),
    );
    world.add(Object::sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian_texture(checker),
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
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Object::moving_sphere(
                        center,
                        center2,
                        0.2,
                        Material::lambertian(albedo),
                    ));
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

    let bvh_node = BVHNode::from_list(&mut world);
    world = HittableList::new();
    world.add(Object::Bvh(bvh_node));

    // Camera
    let config = CameraConfig::default()
        .image_width(400)
        .aspect_ratio(16.0 / 9.0)
        .samples_per_pixel(100)
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

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Texture::checker(
        0.32,
        Texture::solid_color(Color::new(0.2, 0.3, 0.1)),
        Texture::solid_color(Color::new(0.9, 0.9, 0.9)),
    );

    world.add(Object::sphere(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Material::lambertian_texture(checker.clone()),
    ));
    world.add(Object::sphere(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Material::lambertian_texture(checker),
    ));

    let config = CameraConfig::default()
        .image_width(400)
        .aspect_ratio(16.0 / 9.0)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.0)
        .look_from(Point3::new(13.0, 2.0, 3.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .focus_dist(100.0);
    let camera = Camera::initialize(config);
    camera.render(&world);
}

fn earth() {
    let earth_texture = Texture::image("earthmap.jpg");
    let earth_surface = Material::lambertian_texture(earth_texture);
    let globe = Object::sphere(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    let mut world = HittableList::new();
    world.add(globe);

    let config = CameraConfig::default()
        .image_width(400)
        .aspect_ratio(16.0 / 9.0)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.0)
        .look_from(Point3::new(0.0, 0.0, 12.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0);
    let camera = Camera::initialize(config); 
    camera.render(&world);
}

fn main() {
    match 3 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        _ => (),
    }
}
