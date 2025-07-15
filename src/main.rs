use anyhow::{Context, Result};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{DeviceId, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop}; // Import Result and Context
use winit::window::{Window, WindowAttributes, WindowId};
#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl App {
    fn handle_keyboard_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: KeyEvent,
        is_synthetic: bool,
    ) {
        if event.physical_key
            == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape)
        {
            event_loop.exit();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = WindowAttributes::default()
                .with_title("Ladder RS Application")
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

            let window = event_loop
                .create_window(window_attributes)
                .context("Failed to create window")
                .unwrap();
            self.window = Some(window);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                self.handle_keyboard_input(event_loop, device_id, event, is_synthetic);
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();

    match event_loop.run_app(&mut app) {
        Ok(_) => {
            println!("Application exited successfully.");
        }
        Err(e) => {
            eprintln!("Application encountered an error: {}", e);
        }
    }
    Ok(())
}
