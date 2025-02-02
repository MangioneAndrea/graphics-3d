use std::sync::Arc;

use glam::Vec3;

use super::{
    camera::Camera,
    meshes::{Hit, Mesh},
    ray::Ray,
};

fn compute_color<const N: usize>(meshes: &[Arc<dyn Mesh>; N], r: &Ray, max_depth: usize) -> Vec3 {
    if max_depth == 0 {
        return Vec3::new(0., 0., 0.);
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
        let mut dir = hit.normal + super::random_unit_vector();

        if dir.dot(hit.normal) <= 0. {
            dir = -dir;
        }

        return 0.1 * compute_color(meshes, &Ray::new(hit.point, dir), max_depth - 1);
    }

    let dir = r.direction.normalize();

    let a = (dir.y + 1.) / 2.;

    let color = (1.0 - a) * Vec3::new(255., 255., 255.) + a * Vec3::new(127., 180., 255.);

    color
}

#[derive(Clone)]
pub struct RayTracing<const N: usize> {
    spheres: [Arc<dyn Mesh>; N],
    max_depth: usize,
    samples: usize,
    camera: Camera,
}

unsafe impl<const N: usize> Send for RayTracing<N> {}

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
                + ((y + rand::random_range(-0.5..0.5)) * self.camera.delta_u)
                + ((x + rand::random_range(-0.5..0.5)) * self.camera.delta_v)
                - self.camera.center;

            let r = Ray::new(self.camera.center, d);

            color += compute_color(&self.spheres, &r, self.max_depth) / self.samples as f32;
        }

        color
    }
}
