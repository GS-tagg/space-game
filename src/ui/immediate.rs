use crate::math::Vector2D;
use crate::input::InputState;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn contains(&self, point: Vector2D) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

pub struct SimpleUI;

impl SimpleUI {
    /// TASK: Perform hit-test, generate draw primitive, and return click status
    pub fn button(&mut self, _rect: Rect, _label: &str, _input: &InputState) -> bool {
        todo!("Render button primitive and handle click detection")
    }
}
