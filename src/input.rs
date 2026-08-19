#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum KeyCode {
    W, A, S, D, Space, Escape,
}

pub struct InputTracker;

impl InputTracker {
    pub fn new() -> Self {
        Self
    }

    pub fn is_key_down(&self, _key: KeyCode) -> bool {
        todo!("Query key press state")
    }

    pub fn end_frame(&mut self) {
        todo!("Reset single-frame inputs")
    }
}
