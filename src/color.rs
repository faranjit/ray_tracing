use crate::{interval::INTENSITY, vec3::Vec3};

pub type Color = Vec3;

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);

pub fn write_color(pixel_color: Color) {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let r_byte = (256 as f64 * INTENSITY.clamp(r)) as u64;
    let g_byte = (256 as f64 * INTENSITY.clamp(g)) as u64;
    let b_byte = (256 as f64 * INTENSITY.clamp(b)) as u64;

    println!("{} {} {}", r_byte, g_byte, b_byte);
}

#[inline(always)]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
