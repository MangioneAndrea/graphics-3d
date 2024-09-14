use std::rc::Rc;

use softbuffer::Buffer;
use winit::window::Window;

pub fn step(
    mut buffer: Buffer<Rc<Window>, Rc<Window>>,
    width: u32,
    height: u32,
) -> Buffer<Rc<Window>, Rc<Window>> {
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
