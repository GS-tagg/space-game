#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PrimitiveVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

pub struct BatchRenderer;

impl BatchRenderer {
    /// TASK: Add a 2D line segment (orbit line or sensor ring) to batch buffer
    pub fn draw_line(&mut self, _start: [f32; 2], _end: [f32; 2], _color: [f32; 4]) {
        todo!("Push line vertices into primitive array")
    }

    /// TASK: Add a filled rectangle (UI panel) to batch buffer
    pub fn draw_rect(&mut self, _pos: [f32; 2], _size: [f32; 2], _color: [f32; 4]) {
        todo!("Push rectangle quad vertices into primitive array")
    }

    /// TASK: Flush queued vertex data to GPU
    pub fn flush(&mut self) {
        todo!("Upload buffer to GPU and draw")
    }
}
