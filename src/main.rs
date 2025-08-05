use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default)]
struct App<'a> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
}

struct Vector4 {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

const DEFAULT_WIDTH: f32 = 800.0;
const DEFAULT_HEIGHT: f32 = 600.0;

impl<'a> App<'a> {
    fn new() -> Self {
        App {
            window: None,
            pixels: None,
        }
    }

    fn size(&self) -> (f32, f32) {
        if self.window.is_none() {
            return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
        }
        self.window.as_ref().unwrap().inner_size().into()
    }

    fn draw(&mut self, event_loop: &ActiveEventLoop) {
        let size = self.size();

        let frame = self.pixels.as_mut().unwrap().frame_mut();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % size.0 as usize) as f32;
            let y = (i / size.0 as usize) as f32;

            // Example: Fill the screen with a gradient
            let color = Vector4 {
                r: x / size.1 as f32,
                g: y / size.1 as f32,
                b: 0.5,
                a: 1.0,
            };

            pixel[0] = (color.r * 255.0) as u8;
            pixel[1] = (color.g * 255.0) as u8;
            pixel[2] = (color.b * 255.0) as u8;
            pixel[3] = (color.a * 255.0) as u8; // Alpha channel
        }

        // Draw the pixels to the window.
        if let Err(e) = self.pixels.as_ref().unwrap().render() {
            eprintln!("Pixels render error: {}", e);
            event_loop.exit();
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = self.size();
        // It is recommended to recreate all graphics and window contexts on resume.
        let window_attributes = WindowAttributes::default()
            .with_title("Ladder RS")
            .with_inner_size(winit::dpi::LogicalSize::new(size.0, size.1))
            .with_visible(true);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.clone());

        let pixels = Pixels::new(DEFAULT_WIDTH as u32, DEFAULT_HEIGHT as u32, surface_texture)
            .expect("Failed to create Pixels instance");

        self.window = Some(window);
        self.pixels = Some(pixels);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Queue a RedrawRequested event.

                self.draw(event_loop);

                // This queues a redraw request.
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Resize the Pixels instance.
                self.pixels
                    .as_mut()
                    .unwrap()
                    .resize_surface(size.width, size.height)
                    .expect("Failed to resize surface");

                self.pixels
                    .as_mut()
                    .unwrap()
                    .resize_buffer(size.width, size.height)
                    .expect("Failed to resize buffer");

                // Request a redraw after resizing.
                self.window.as_ref().unwrap().request_redraw();
            }

            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    let _ = event_loop.run_app(&mut app);
}
