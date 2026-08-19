use crate::game::types::Vector2D;

pub struct MapCamera {
    pub center_world_pos: Vector2D,
    pub zoom_level: f64,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl MapCamera {
    /// TASK: Convert a space coordinate into screen pixel space
    pub fn world_to_screen(&self, _world_pos: Vector2D) -> (f32, f32) {
        todo!("Calculate screen pixel coordinates from world position")
    }

    /// TASK: Convert mouse click pixel coordinates back into space world coordinates
    pub fn screen_to_world(&self, _screen_x: f32, _screen_y: f32) -> Vector2D {
        todo!("Calculate world position from screen click coordinates")
    }
}
