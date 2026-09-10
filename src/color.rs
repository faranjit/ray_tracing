use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r_byte = (255.999 * r) as u32;
    let g_byte = (255.999 * g) as u32;
    let b_byte = (255.999 * b) as u32;

    println!("{} {} {}", r_byte, g_byte, b_byte);
}