#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleUniform {
    color: [f32; 4],
}

impl TriangleUniform {
    pub fn new(color: [f32; 4]) -> Self {
        Self { color }
    }
}
