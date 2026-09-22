use std::io::{self, Write};

use crate::{interval::INTENSITY, rtweekend::Real, vec3::Vec3};

pub type Color = Vec3;

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);

pub fn write_color(out: &mut impl Write, pixel_color: Color) -> io::Result<()> {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let r_byte = (256 as Real * INTENSITY.clamp(r)) as u64;
    let g_byte = (256 as Real * INTENSITY.clamp(g)) as u64;
    let b_byte = (256 as Real * INTENSITY.clamp(b)) as u64;

    writeln!(out, "{} {} {}", r_byte, g_byte, b_byte)
}

#[inline]
pub fn linear_to_gamma(linear_component: Real) -> Real {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
