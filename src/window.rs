use crate::config::{config, load_runtime_config, save_runtime_config};
use crate::input::InputState;
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
    input: InputState,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = WindowAttributes::default()
                .with_title("space-game")
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

            let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
            let gpu = GpuState::new(window.clone(), &[] as &[Vertex]);
            self.window = Some(window);
            self.gpu = Some(gpu);
            let cfg = config();
            self.fps = Some(FpsTracker::new(cfg.target_fps as f32));
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

        self.input.update(&event);

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(physical_size) => {
                gpu.resize(physical_size);
            }

            WindowEvent::RedrawRequested => {
                let cfg = config();
                let start = fps.begin_render();
                if let Err(e) = gpu.render() {
                    eprintln!("Render error: {e:?}");
                }

                fps.end_render(start);

                if cfg.fps_tracker_enabled {
                    fps.tick();
                }

                self.input.end_frame();
                window.request_redraw();
            }

            _ => {}
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    load_runtime_config()?;

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        window: None,
        gpu: None,
        fps: None,
        input: InputState::new(),
    };
    event_loop.run_app(&mut app)?;

    save_runtime_config()?;
    Ok(())
}
