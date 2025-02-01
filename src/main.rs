pub mod utils;
mod views;

use std::{num::NonZeroU32, rc::Rc};

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = Application::default();

    event_loop.run_app(&mut app)?;

    println!("main loop exit");
    return Ok(());
}

struct Application {
    window: Option<Rc<Window>>,
    renderer: Box<dyn views::View>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            window: None,
            renderer: Box::new(views::ColorsView),
        }
    }
}

impl ApplicationHandler for Application {
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = self.window.clone().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => match event.logical_key.as_ref() {
                winit::keyboard::Key::Character("1") => self.renderer = Box::new(views::ColorsView),
                winit::keyboard::Key::Character("2") => {
                    self.renderer = Box::new(views::RayTracingView)
                }
                winit::keyboard::Key::Character("q") => std::process::exit(0),
                _ => {}
            },
            WindowEvent::RedrawRequested => {
                let context = softbuffer::Context::new(window.clone()).unwrap();
                let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

                let size = self.window.clone().unwrap().inner_size();

                let width = size.width;
                let height = size.height;

                surface
                    .resize(
                        NonZeroU32::new(width as _).unwrap(),
                        NonZeroU32::new(height as _).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();

                self.renderer.step(&mut buffer, width, height);

                buffer.present().unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }

            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("resumed");
        let window_attributes = Window::default_attributes()
            .with_title(self.renderer.get_name())
            .with_inner_size(LogicalSize::new(800, 600));

        self.window = Some(Rc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create a window"),
        ));
    }
}
