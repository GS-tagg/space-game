pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub struct WindowState;

impl WindowState {
    pub fn new(_config: WindowConfig) -> Self {
        todo!("Initialize winit window context")
    }

    pub fn run<F>(&mut self, _frame_callback: F)
    where
        F: FnMut(f32),
    {
        todo!("Implement winit event loop processing")
    }
}
