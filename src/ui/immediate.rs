use crate::math::Vector2D;
use crate::input::InputTracker;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn contains(&self, _point: Vector2D) -> bool {
        todo!("Implement point-in-rectangle check")
    }
}

pub struct SimpleUI;

impl SimpleUI {
    /// TASK: Perform hit-test, generate draw primitive, and return click status
    pub fn button(&mut self, _rect: Rect, _label: &str, _input: &InputTracker) -> bool {
        todo!("Render button primitive and handle click detection")
    }
}
