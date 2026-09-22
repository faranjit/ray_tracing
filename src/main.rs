mod aabb;
mod bvh_node;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod perlin;
mod ray;
mod rtw_image;
mod rtweekend;
mod texture;
mod vec3;

use crate::{
    bvh_node::BVHNode,
    camera::{Camera, CameraConfig},
    color::{BLACK, Color},
    hittable::{HittableList, Object},
    material::Material,
    rtweekend::{Real, random_double, random_double_range},
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
                (a as Real) + 0.9 * random_double(),
                0.2,
                (b as Real) + 0.9 * random_double(),
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
    world.add(bvh_node);

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
        .focus_dist(10.0)
        .background(Color::new(0.7, 0.8, 1.0));
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
        .focus_dist(100.0)
        .background(Color::new(0.7, 0.8, 1.0));
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

fn perlin_spheres() {
    let mut world = HittableList::new();

    let pertext = Texture::noise(4.0);
    world.add(Object::sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian_texture(pertext.clone()),
    ));
    world.add(Object::sphere(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Material::lambertian_texture(pertext),
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
        .background(Color::new(0.7, 0.8, 1.0));
    let camera = Camera::initialize(config);
    camera.render(&world);
}

fn quads() {
    let mut world = HittableList::new();

    let left_red = Material::lambertian(Color::new(1.0, 0.2, 0.2));
    let back_green = Material::lambertian(Color::new(0.2, 1.0, 0.2));
    let right_blue = Material::lambertian(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Material::lambertian(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Material::lambertian(Color::new(0.2, 0.8, 0.8));

    world.add(Object::quad(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));

    world.add(Object::quad(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));

    world.add(Object::quad(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));

    world.add(Object::quad(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));

    world.add(Object::quad(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    let config = CameraConfig::default()
        .image_width(400)
        .aspect_ratio(1.0)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(80.0)
        .look_from(Point3::new(0.0, 0.0, 9.0))
        .look_at(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .background(Color::new(0.7, 0.8, 1.0));
    let camera = Camera::initialize(config);
    camera.render(&world);
}

fn simple_light() {
    let mut world = HittableList::new();

    let pertext = Texture::noise(4.0);
    world.add(Object::sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian_texture(pertext.clone()),
    ));

    world.add(Object::sphere(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Material::lambertian_texture(pertext),
    ));

    let diff_light = Material::diffuse_light(Color::new(4.0, 4.0, 4.0));
    world.add(Object::sphere(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diff_light.clone(),
    ));

    world.add(Object::quad(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diff_light,
    ));

    let config = CameraConfig::default()
        .image_width(400)
        .aspect_ratio(16.0 / 9.0)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.0)
        .look_from(Point3::new(26.0, 3.0, 6.0))
        .look_at(Point3::new(0.0, 2.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .background(BLACK);
    let camera = Camera::initialize(config);
    camera.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05));
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15));
    let light = Material::diffuse_light(Color::new(15.0, 15.0, 15.0));

    world.add(Object::quad(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Object::quad(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Object::quad(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    world.add(Object::quad(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Object::quad(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(Object::quad(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let mut box1 = Object::box_3d(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Object::rotate_y(box1, 15.0);
    box1 = Object::translate(box1, Vec3::new(265.0, 0.0, 295.0));
    world.add(box1);

    let mut box2 = Object::box_3d(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    );
    box2 = Object::rotate_y(box2, -18.0);
    box2 = Object::translate(box2, Vec3::new(130.0, 0.0, 65.0));
    world.add(box2);

    let config = CameraConfig::default()
        .image_width(600)
        .aspect_ratio(1.0)
        .samples_per_pixel(200)
        .max_depth(50)
        .vfov(40.0)
        .look_from(Point3::new(278.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .background(BLACK);
    let camera = Camera::initialize(config);
    camera.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05));
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15));
    let light = Material::diffuse_light(Color::new(7.0, 7.0, 7.0));

    world.add(Object::quad(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Object::quad(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Object::quad(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    ));
    world.add(Object::quad(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Object::quad(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Object::quad(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let box1 = Object::box_3d(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Object::translate(Object::rotate_y(box1, 15.0), Vec3::new(265.0, 0.0, 295.0));

    let box2 = Object::box_3d(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Object::translate(Object::rotate_y(box2, -18.0), Vec3::new(130.0, 0.0, 65.0));

    world.add(Object::medium_color(box1, 0.01, BLACK));
    world.add(Object::medium_color(box2, 0.01, Color::new(1.0, 1.0, 1.0)));

    let config = CameraConfig::default()
        .image_width(600)
        .aspect_ratio(1.0)
        .samples_per_pixel(200)
        .max_depth(50)
        .vfov(40.0)
        .look_from(Point3::new(278.0, 278.0, -800.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .background(BLACK);
    let camera = Camera::initialize(config);
    camera.render(&world);
}

fn final_scene(image_width: u64, samples_per_pixel: u32, max_depth: u16) {
    let ground = Material::lambertian(Color::new(0.48, 0.83, 0.53));

    let mut boxes1 = HittableList::new();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as Real) * w;
            let z0 = -1000.0 + (j as Real) * w;
            boxes1.add(Object::box_3d(
                Point3::new(x0, 0.0, z0),
                Point3::new(x0 + w, random_double_range(1.0, 101.0), z0 + w),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();
    world.add(BVHNode::from_list(&mut boxes1));

    let light = Material::diffuse_light(Color::new(7.0, 7.0, 7.0));
    world.add(Object::quad(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    ));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    world.add(Object::moving_sphere(
        center1,
        center2,
        50.0,
        Material::lambertian(Color::new(0.7, 0.3, 0.1)),
    ));

    world.add(Object::sphere(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Material::dielectric(1.5),
    ));
    world.add(Object::sphere(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Material::metal(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    // Glass sphere: both as a surface and as a blue smoke inside
    world.add(Object::sphere(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Material::dielectric(1.5),
    ));
    world.add(Object::medium_color(
        Object::sphere(
            Point3::new(360.0, 150.0, 145.0),
            70.0,
            Material::dielectric(1.5),
        ),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));

    // thin smoke around the whole scene
    world.add(Object::medium_color(
        Object::sphere(
            Point3::new(0.0, 0.0, 0.0),
            5000.0,
            Material::dielectric(1.5),
        ),
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    ));

    world.add(Object::sphere(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        Material::lambertian_texture(Texture::image("earthmap.jpg")),
    ));
    world.add(Object::sphere(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Material::lambertian_texture(Texture::noise(0.2)),
    ));

    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let mut boxes2 = HittableList::new();
    for _ in 0..1000 {
        boxes2.add(Object::sphere(
            Vec3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }
    let cluster = BVHNode::from_list(&mut boxes2);
    world.add(Object::translate(
        Object::rotate_y(cluster, 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    let config = CameraConfig::default()
        .image_width(image_width)
        .aspect_ratio(1.0)
        .samples_per_pixel(samples_per_pixel)
        .max_depth(max_depth)
        .vfov(40.0)
        .look_from(Point3::new(478.0, 278.0, -600.0))
        .look_at(Point3::new(278.0, 278.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .background(BLACK);
    let camera = Camera::initialize(config);
    camera.render(&world);
}

fn main() {
    match 9 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        _ => final_scene(400, 400, 20),
    }
}
