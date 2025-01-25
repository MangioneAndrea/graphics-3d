use std::rc::Rc;

use softbuffer::Buffer;
use winit::window::Window;

use crate::View;

pub struct RayTracingView;

impl View for RayTracingView {
    fn get_name(&self) -> &'static str {
        "Ray Tracing"
    }

    fn step<'a>(
        &mut self,
        mut buffer: Buffer<'a, Rc<Window>, Rc<Window>>,
        width: u32,
        height: u32,
    ) -> Buffer<'a, Rc<Window>, Rc<Window>> {
        let aspect_ratio = width as f32 / height as f32;

        let focal_length = 1.;
        let viewport_height = 2.;

        for index in 0..(width * height) {
            let y = index as f32 / width as f32;
            let x = index % width;
            let red = (x as f32 / width as f32 * 255.) as u32;
            let green = (y as f32 / height as f32 * 255.) as u32;
            let blue = 0; //(x * y as u32) % 255;

            buffer[index as usize] = blue | (green << 8) | (red << 16);
        }

        buffer
    }
}
