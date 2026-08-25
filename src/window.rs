use crate::renderer::{fps::FpsTracker, gpu::GpuState, gpu::Vertex};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes};
#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    gpu: Option<GpuState>,
    fps: Option<FpsTracker>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = WindowAttributes::default()
                .with_title("Winit + Wgpu App")
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

            let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
            let triangle = [
                Vertex {
                    position: [-1.0, -1.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.0, 1.0],
                    color: [0.0, 0.0, 0.0],
                },
            ];

            let gpu = GpuState::new(window.clone(), &triangle);

            self.window = Some(window);
            self.gpu = Some(gpu);
            self.fps = Some(FpsTracker::new(60.0));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let (Some(gpu), Some(window), Some(fps)) =
            (self.gpu.as_mut(), self.window.as_ref(), self.fps.as_mut())
        else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(physical_size) => {
                gpu.resize(physical_size);
            }

            WindowEvent::RedrawRequested => {
                let start = fps.begin_render();

                if let Err(e) = gpu.render() {
                    eprintln!("Render error: {e:?}");
                }

                fps.end_render(start);
                fps.tick();

                window.request_redraw();
            }

            _ => {}
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
