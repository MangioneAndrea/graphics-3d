use std::{rc::Rc, sync::mpsc::Sender, time::Duration};

use glam::Vec3;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use softbuffer::Buffer;
use winit::window::Window;

use crate::{
    utils::{
        self,
        colors::{self, vec3àto_color},
        ray::Ray,
        sphere::{Hit, Sphere},
    },
    ScreenChunk,
};

pub struct RayTracingView {
    spheres: Vec<Sphere>,
    samples: usize,
    max_depth: usize,
}

impl Default for RayTracingView {
    fn default() -> Self {
        Self {
            spheres: vec![
                Sphere::new(Vec3::new(0., 0., -1.), 0.5),
                Sphere::new(Vec3::new(0., -100.5, -1.), 100.),
            ],
            samples: 10,
            max_depth: 100,
        }
    }
}

fn color(spheres: &Vec<Sphere>, r: &Ray, max_depth: usize) -> Vec3 {
    //if max_depth <= 0 {
    //    return Vec3::new(0., 0., 0.);
    //}
    let mut closest_sphere: Option<&Sphere> = None;
    let mut closest_hit: Option<Hit> = None;

    for s in spheres {
        let t = closest_hit.clone().map(|h| h.t).unwrap_or(f32::INFINITY);

        if let Some(t) = s.hit(r, 0., t) {
            closest_sphere = Some(s);
            closest_hit = Some(t);
        }
    }

    if let Some(t) = closest_hit {
        let mut dir = utils::random_unit_vector();

        if dir.dot(t.normal) <= 0. {
            dir = -dir;
        }
        // return 0.5 * color(spheres, &Ray::new(t.p, dir), max_depth - 1);
        return colors::from_unit_vec(0.5 * (t.normal + 1.));
    }

    let dir = r.direction.normalize();

    let a = (dir.y + 1.) / 2.;

    let color = (1.0 - a) * Vec3::new(255., 255., 255.) + a * Vec3::new(127., 180., 255.);

    color
}
impl RayTracingView {}

impl super::View for RayTracingView {
    fn get_name(&self) -> &'static str {
        "Ray Tracing"
    }

    fn step<'a>(&mut self, buffer: Sender<ScreenChunk>, width: u32, height: u32) {
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

        let spheres = Box::leak(Box::new(self.spheres.clone()));
        let samples = self.samples;
        let max_depth = self.max_depth;

        let mut sc = ScreenChunk {
            from: 0,
            data: vec![],
        };

        for index in 0..(width * height) {
            let x = index as f32 / width as f32;
            let y = index as f32 % width as f32;

            let center = upper_center + (y * delta_u) + (x * delta_v);

            let mut c = Vec3::new(0., 0., 0.);

            for _ in 0..samples {
                let d = upper_left
                    + ((y + rand::random_range(-0.5..0.5)) * delta_u)
                    + ((x + rand::random_range(-0.5..0.5)) * delta_v)
                    - camera_center;

                let r = Ray::new(center, d);

                c += color(&spheres, &r, max_depth);
            }

            let color = c / samples as f32;

            sc.data.push(vec3àto_color(&color))
        }

        buffer.send(sc).unwrap()
    }
}
