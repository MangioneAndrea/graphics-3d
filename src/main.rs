use std::rc::Rc;

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
}

impl Default for Application {
    fn default() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for Application {
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = self.window.clone().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let context = softbuffer::Context::new(window.clone()).unwrap();
                let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }

            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("resumed");
        let window_attributes = Window::default_attributes()
            .with_title("hi")
            .with_inner_size(LogicalSize::new(800., 600.));

        self.window = Some(Rc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create a window"),
        ));
    }
}
