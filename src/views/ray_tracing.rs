use std::rc::Rc;

use glam::Vec3;
use softbuffer::Buffer;
use winit::window::Window;

use crate::utils::{
    colors::{self, vec3àto_color},
    ray::Ray,
    sphere::{Hit, Sphere},
};

pub struct RayTracingView {
    spheres: Vec<Sphere>,
    samples: usize,
}

impl Default for RayTracingView {
    fn default() -> Self {
        Self {
            spheres: vec![
                Sphere::new(Vec3::new(0., 0., -1.), 0.5),
                Sphere::new(Vec3::new(0., -101., -1.), 100.),
            ],
            samples: 10,
        }
    }
}

impl RayTracingView {
    fn color(&self, r: &Ray) -> Vec3 {
        let mut closest_sphere: Option<&Sphere> = None;
        let mut closest_hit: Option<Hit> = None;

        for s in &self.spheres {
            let t = closest_hit.clone().map(|h| h.t).unwrap_or(f32::INFINITY);

            if let Some(t) = s.hit(r, 0., t) {
                closest_sphere = Some(s);
                closest_hit = Some(t);
            }
        }

        if let Some(t) = closest_hit {
            return colors::from_unit_vec(0.5 * (t.normal + 1.));
        }

        let dir = r.direction.normalize();

        let a = (dir.y + 1.) / 2.;

        let color = (1.0 - a) * Vec3::new(255., 255., 255.) + a * Vec3::new(127., 180., 255.);

        color
    }
}

impl super::View for RayTracingView {
    fn get_name(&self) -> &'static str {
        "Ray Tracing"
    }

    fn step<'a>(
        &mut self,
        buffer: &mut Buffer<'a, Rc<Window>, Rc<Window>>,
        width: u32,
        height: u32,
    ) {
        let aspect_ratio = width as f32 / height as f32;

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * aspect_ratio;

        let camera_center = Vec3::new(0., 0., 0.);

        let u = Vec3::new(viewport_width, 0., 0.);
        let v = Vec3::new(0., -viewport_height, 0.);

        let delta_u = u / width as f32;
        let delta_v = v / height as f32;

        let upper_left = camera_center - Vec3::new(0., 0., focal_length) - u / 2. - v / 2.;

        let upper_center = upper_left + 0.5 * (delta_v + delta_u); // pixel00

        for index in 0..(width * height) {
            let x = index as f32 / width as f32;
            let y = index as f32 % width as f32;

            let center = upper_center + (y * delta_u) + (x * delta_v);

            let mut color = Vec3::new(0., 0., 0.);

            for _ in 0..self.samples {
                let d = upper_left
                    + ((y + rand::random_range(-0.5..0.5)) * delta_u)
                    + ((x + rand::random_range(-0.5..0.5)) * delta_v)
                    - camera_center;

                let r = Ray::new(center, d);

                let c = self.color(&r);

                color += c;
            }

            let color = color / self.samples as f32;

            buffer[index as usize] = vec3àto_color(&color);
        }
    }
}
