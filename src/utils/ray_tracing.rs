use glam::Vec3;

use super::{
    camera::Camera,
    ray::Ray,
    sphere::{Hit, Sphere},
};

fn compute_color<const N: usize>(spheres: &[Sphere; N], r: &Ray, max_depth: usize) -> Vec3 {
    if max_depth == 0 {
        return Vec3::new(0., 0., 0.);
    }
    let mut closest_sphere: Option<&Sphere> = None;
    let mut closest_hit: Option<Hit> = None;

    for s in spheres {
        let t = closest_hit.clone().map(|h| h.t).unwrap_or(f32::INFINITY);

        if let Some(hit) = s.hit(r, 0.001, t) {
            closest_sphere = Some(s);
            closest_hit = Some(hit);
        }
    }

    if let Some(t) = closest_hit {
        let mut dir = t.normal + super::random_unit_vector();

        if dir.dot(t.normal) <= 0. {
            dir = -dir;
        }

        return 0.1 * compute_color(spheres, &Ray::new(t.p, dir), max_depth - 1);
    }

    let dir = r.direction.normalize();

    let a = (dir.y + 1.) / 2.;

    let color = (1.0 - a) * Vec3::new(255., 255., 255.) + a * Vec3::new(127., 180., 255.);

    color
}

#[derive(Clone, Copy)]
pub struct RayTracing<const N: usize> {
    spheres: [Sphere; N],
    max_depth: usize,
    samples: usize,
    camera: Camera,
}

impl<const N: usize> RayTracing<N> {
    pub fn new(spheres: [Sphere; N], camera: Camera, max_depth: usize, samples: usize) -> Self {
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
