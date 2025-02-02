use glam::Vec3;

pub mod camera;
pub mod colors;
pub mod materials;
pub mod ray;
pub mod ray_tracing;
pub mod meshes;

#[must_use]
pub fn random_unit_vector() -> Vec3 {
    // return Vec3::new(0.3, 0.3, 0.3);
    loop {
        let p = Vec3::new(
            rand::random_range(-1.0..1.),
            rand::random_range(-1.0..1.),
            rand::random_range(-1.0..1.),
        );

        let ls = p.length_squared();

        if (1e-90 as f32) < ls && ls <= 1. {
            return p / ls.sqrt();
        }
    }
}
