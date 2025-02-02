use glam::Vec3;

use super::ray::Ray;

pub mod sphere;

#[derive(Debug, Clone)]
pub struct Hit {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

pub trait Mesh: 'static{
    fn hit(&self, ray: &Ray, ray_t_min: f32, ray_t_max: f32) -> Option<Hit>;
}
