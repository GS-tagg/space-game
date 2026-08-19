pub struct Camera {
    pub position: [f32; 3],
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        todo!("Initialize camera transform properties")
    }

    pub fn build_view_matrix(&self) -> [[f32; 4]; 4] {
        todo!("Return view matrix")
    }
}
