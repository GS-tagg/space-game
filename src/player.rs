use crate::input::InputTracker;

pub struct PlayerController {
    pub position: [f32; 3],
    pub move_speed: f32,
}

impl PlayerController {
    pub fn new(start_pos: [f32; 3]) -> Self {
        todo!("Initialize player state")
    }

    pub fn update(&mut self, _input: &InputTracker, _delta_time: f32) {
        todo!("Update player coordinates based on input")
    }
}
