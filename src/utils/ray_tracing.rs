use std::sync::Arc;

use glam::Vec3;

use super::{
    camera::Camera,
    colors::{self, SKY_BLUE, WHITE},
    meshes::{Hit, Mesh},
    ray::Ray,
};

fn compute_color<const N: usize>(meshes: &[Arc<dyn Mesh>; N], r: &Ray, max_depth: usize) -> Vec3 {
    if max_depth == 0 {
        return colors::BLACK;
    }
    let mut _closest_mesh: Option<&_> = None;
    let mut closest_hit: Option<Hit> = None;

    for s in meshes {
        let distance = closest_hit
            .clone()
            .map(|h| h.distance)
            .unwrap_or(f32::INFINITY);

        if let Some(hit) = s.hit(r, 0.001, distance) {
            _closest_mesh = Some(s);
            closest_hit = Some(hit);
        }
    }

    if let Some(hit) = closest_hit {
        let (scattered, attenuation) = hit.material.scatter(r, &hit);

        return attenuation * compute_color(meshes, &scattered, max_depth - 1);
    }

    let dir = r.direction.normalize();

    let a = (dir.y + 1.) / 2.;

    let color = (1.0 - a) * WHITE + a * SKY_BLUE;

    color
}

#[derive(Clone)]
pub struct RayTracing<const N: usize> {
    spheres: [Arc<dyn Mesh>; N],
    max_depth: usize,
    samples: usize,
    camera: Camera,
}

impl<const N: usize> RayTracing<N> {
    pub fn new(
        spheres: [Arc<dyn Mesh>; N],
        camera: Camera,
        max_depth: usize,
        samples: usize,
    ) -> Self {
        Self {
            spheres,
            max_depth,
            samples,
            camera,
        }
    }

    pub fn compute_pixel(&self, x: f32, y: f32) -> Vec3 {
        let mut color = Vec3::new(0., 0., 0.);

        for _ in 0..self.samples {
            let d = self.camera.upper_left
                + ((x + rand::random_range(-0.5..0.5)) * self.camera.delta_u)
                + ((y + rand::random_range(-0.5..0.5)) * self.camera.delta_v)
                - self.camera.center;

            let r = Ray::new(self.camera.center, d);

            color += compute_color(&self.spheres, &r, self.max_depth) / self.samples as f32;
        }

        color
    }
}
