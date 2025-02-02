use glam::Vec3;

pub mod colors;
pub mod ray;
pub mod sphere;

#[must_use]
pub fn random_unit_vector() -> Vec3 {
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
