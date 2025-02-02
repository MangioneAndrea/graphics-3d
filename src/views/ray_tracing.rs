use std::sync::{mpsc::Sender, Arc};

use glam::Vec3;

use crate::{
    utils::{
        camera::Camera,
        colors::vec3àto_color,
        meshes::{sphere::Sphere, Mesh},
        ray_tracing::RayTracing,
    },
    ScreenChunk,
};

pub struct RayTracingView {
    meshes: [Arc<dyn Mesh>; 2],
    samples: usize,
    max_depth: usize,
}

impl Default for RayTracingView {
    fn default() -> Self {
        Self {
            meshes: [
                Arc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
                Arc::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
            ],
            samples: 10,
            max_depth: 10,
        }
    }
}

impl RayTracingView {}

impl super::View for RayTracingView {
    fn get_name(&self) -> &'static str {
        "Ray Tracing"
    }

    fn step<'a>(&mut self, buffer: Sender<ScreenChunk>, width: u32, height: u32) {
        let camera = Camera::new(Vec3::new(0., 0., 0.), width, height);

        let samples = self.samples;
        let max_depth = self.max_depth;

        let threads = std::thread::available_parallelism()
            .expect("Windows macos and linux know the amount of threads")
            .get();

        let step_size = (width * height) as usize / threads;

        let rt = RayTracing::<2>::new(self.meshes.clone(), camera, samples, max_depth);

        for t in 0..threads {
            let buffer = buffer.clone();

            let rt = rt.clone();

            std::thread::spawn(move || {
                let mut sc = ScreenChunk {
                    from: t as usize * step_size as usize,
                    data: vec![],
                };

                for index in t * step_size..(t + 1) * step_size {
                    let x = index as f32 / width as f32;
                    let y = index as f32 % width as f32;

                    let color = rt.compute_pixel(x, y);

                    sc.data.push(vec3àto_color(&color))
                }

                buffer.send(sc).unwrap();
            });
        }
    }
}
