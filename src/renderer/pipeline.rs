#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

pub struct ShaderPipeline;

impl ShaderPipeline {
    /// TASK: Read GLSL/WGSL code, compile shaders, and build graphics pipeline
    pub fn new(_vertex_source: &str, _fragment_source: &str) -> Self {
        todo!("Compile shaders and configure depth/blend state")
    }
}
