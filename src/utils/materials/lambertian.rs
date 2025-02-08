use std::sync::Arc;

use crate::utils::{
    colors::{self, ColorVec},
    random_unit_vector,
    ray::Ray,
};

use super::Material;

#[derive(Debug)]
pub struct Lambertian(ColorVec);

impl Lambertian {
    pub fn new(color: ColorVec) -> Arc<Self> {
        Arc::new(Self(colors::vec3_to_scalar(&color)))
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &crate::utils::ray::Ray,
        hit: &crate::utils::meshes::Hit,
    ) -> (crate::utils::ray::Ray, colors::ColorVec) {
        let mut direction = hit.normal + random_unit_vector();

        if direction.abs().element_sum() < 1e-7 {
            direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, direction);
        let attenuation = self.0;

        (scattered, attenuation)
    }
}
