use std::fmt::Debug;

use super::{colors, meshes::Hit, ray::Ray};

pub mod lambertian;
pub mod metal;

pub trait Material: Debug {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> (Ray, colors::ColorVec);
}
