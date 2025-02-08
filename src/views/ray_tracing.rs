use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    usize,
};

use glam::Vec3;

use crate::{
    utils::{
        camera::Camera,
        colors::{vec3àto_color, BLUE, GREEN, RED, YELLOW},
        materials::{lambertian::Lambertian, metal::Metal},
        meshes::{sphere::Sphere, Mesh},
        ray_tracing::RayTracing,
    },
    ScreenChunk,
};

const MESHES_COUNT: usize = 4;
pub struct RayTracingView {
    meshes: [Arc<dyn Mesh>; MESHES_COUNT],
    samples: usize,
    max_depth: usize,
}

impl Default for RayTracingView {
    fn default() -> Self {
        Self {
            meshes: [
                Arc::new(Sphere::new(
                    Vec3::new(0., 0., -1.2),
                    0.5,
                    Lambertian::new(BLUE),
                )),
                Arc::new(Sphere::new(Vec3::new(1., 0., -1.), 0.5, Metal::new(YELLOW))),
                Arc::new(Sphere::new(Vec3::new(-1., 0., -1.), 0.5, Metal::new(RED))),
                Arc::new(Sphere::new(
                    Vec3::new(0., -100.5, -1.),
                    100.,
                    Lambertian::new(GREEN),
                )),
            ],
            samples: 5,
            max_depth: 10,
        }
    }
}

impl RayTracingView {}

impl super::View for RayTracingView {
    fn step(&mut self, buffer: Sender<ScreenChunk>, width: u32, height: u32) {
        let camera = Camera::new(Vec3::new(0., 0., 0.), width, height);

        let samples = self.samples;
        let max_depth = self.max_depth;

        let threads = std::thread::available_parallelism()
            .expect("Windows macos and linux know the amount of threads")
            .get();

        let rt = RayTracing::<MESHES_COUNT>::new(self.meshes.clone(), camera, samples, max_depth);

        // Some threads are faster, so they can do multiple rows
        let rows: Arc<Mutex<Vec<_>>> =
            Arc::new(Mutex::new((0..height as usize).into_iter().collect()));

        for _ in 0..threads {
            let rows = rows.clone();
            let buffer = buffer.clone();

            let rt = rt.clone();

            // Draw row by row to allow multithreading
            std::thread::spawn(move || loop {
                let y = {
                    let mut rows = rows.lock().unwrap();
                    if let Some(row) = rows.pop() {
                        row
                    } else {
                        break;
                    }
                };

                let mut sc = ScreenChunk {
                    from: y * width as usize,
                    data: vec![],
                };

                for x in 0..width {
                    let color = rt.compute_pixel(x as f32, y as f32);
                    sc.data.push(vec3àto_color(&color))
                }

                buffer.send(sc).unwrap();
            });
        }
    }
}
