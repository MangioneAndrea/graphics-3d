mod colors;
mod ray_tracing;

use std::rc::Rc;

pub use colors::*;
pub use ray_tracing::*;
use winit::window::Window;

pub trait View {
    fn get_name(&self) -> &'static str;

    fn step<'a>(
        &mut self,
        buffer: &mut softbuffer::Buffer<'a, Rc<Window>, Rc<Window>>,
        width: u32,
        height: u32,
    );
}
