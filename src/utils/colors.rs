use glam::Vec3;

pub type Color = u32;

pub const fn vec3àto_color(v: &Vec3) -> Color {
    v.z.clamp(0., 255.) as u32
        | ((v.y.clamp(0., 255.) as u32) << 8)
        | ((v.x.clamp(0., 255.) as u32) << 16)
}

pub fn from_unit_vec(v: Vec3) -> Vec3 {
    v * 255.
}

pub const RED: u32 = vec3àto_color(&Vec3::new(255., 0., 0.));
