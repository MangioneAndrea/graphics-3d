use std::sync::Arc;

use crate::utils::{
    colors::{self, ColorVec},
    ray::Ray,
};

use super::Material;

#[derive(Debug)]
pub struct Metal(ColorVec);

impl Metal {
    pub fn new(color: ColorVec) -> Arc<Self> {
        Arc::new(Self(colors::vec3_to_scalar(&color)))
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &crate::utils::ray::Ray,
        hit: &crate::utils::meshes::Hit,
    ) -> (crate::utils::ray::Ray, colors::ColorVec) {
        let reflected = ray.direction - 2. * ray.direction.dot(hit.normal) * hit.normal;

        let scattered = Ray::new(hit.point, reflected);
        let attenuation = self.0;

        (scattered, attenuation)
    }
}
