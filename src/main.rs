#![feature(test, const_black_box)]

pub mod utils;
mod views;

use std::{
    num::NonZeroU32,
    sync::{mpsc::TryRecvError, Arc, Mutex},
    time::Duration,
};

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
    window: Option<Arc<Window>>,
    renderer: Box<dyn views::View>,
    outer_buffer: Arc<Mutex<Vec<u32>>>,
    thread_id: Arc<Mutex<usize>>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            window: None,
            renderer: Box::new(views::RayTracingView::default()),
            outer_buffer: Arc::new(Mutex::new(vec![])),
            thread_id: Arc::new(Mutex::new(0)),
        }
    }
}

struct ScreenChunk {
    from: usize,
    data: Vec<u32>,
}

impl Application {
    fn reload_scene(&mut self) {
        /*
         * When reloading a scene, the old buffer can be cleaned up
         * This will cascade through following threads as they can be killed
         *
         * The tracking is done just by simply counting the thread started here,
         * by canging scene the id is increased, so the threads can be stopped.
         */
        let thread_id = self.thread_id.clone();
        let mut thread_id_inner = thread_id.lock().unwrap();
        *thread_id_inner += 1;
        let assigned_id = *thread_id_inner;
        drop(thread_id_inner);

        let size = self.window.clone().unwrap().inner_size();

        let width = size.width;
        let height = size.height;

        {
            let mut v = vec![];
            v.resize(
                (width * height)
                    .try_into()
                    .expect("Width and height must be non negative"),
                0u32,
            );

            self.outer_buffer = Arc::new(Mutex::new(v));
        }

        let (tx, rx) = std::sync::mpsc::channel::<ScreenChunk>();

        self.renderer.step(tx, width, height);

        let buffer = self.outer_buffer.clone();
        let window = self.window.clone();
        std::thread::spawn(move || loop {
            {
                // The
                let thread_id = thread_id.clone();
                if assigned_id != *thread_id.lock().unwrap() {
                    break;
                };
            }
            match rx.try_recv() {
                Ok(chunk) => {
                    buffer.lock().unwrap()[chunk.from..][..chunk.data.len()]
                        .copy_from_slice(chunk.data.as_slice());
                }
                Err(TryRecvError::Empty) => {
                    window.as_ref().unwrap().request_redraw();
                }
                Err(TryRecvError::Disconnected) => {
                    window.as_ref().unwrap().request_redraw();
                    break;
                }
            };
        });
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
            WindowEvent::KeyboardInput { event, .. } => {
                match event.logical_key.as_ref() {
                    winit::keyboard::Key::Character("1") => {
                        self.renderer = Box::new(views::ColorsView);
                    }
                    winit::keyboard::Key::Character("2") => {
                        self.renderer = Box::new(views::RayTracingView::default());
                    }
                    winit::keyboard::Key::Character("q") => std::process::exit(0),
                    _ => {}
                };
                self.reload_scene()
            }
            WindowEvent::RedrawRequested => {
                let tmp_buf = self.outer_buffer.clone();
                let tmp_buf = tmp_buf.lock().unwrap();

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

                if buffer.len() != tmp_buf.len() {
                    return;
                }
                buffer.copy_from_slice(&tmp_buf);
                buffer.present().unwrap();
                // Do not care of more than 60 fps
                std::thread::sleep(Duration::from_millis(1000 / 60));
            }

            WindowEvent::Resized(_) => {
                self.reload_scene();
            }
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("resumed");
        let window_attributes = Window::default_attributes()
            .with_title("press: q to quit | 1 colors | 2 ray tracing")
            .with_inner_size(LogicalSize::new(800, 600));

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create a window"),
        );
        self.window = Some(window.clone());

        self.reload_scene()
    }
}
