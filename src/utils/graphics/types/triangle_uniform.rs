#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleUniform {
    color: [f32; 4],
    resolution: [f32; 2],
    pub translation: [f32; 2],
    pub rotation: [f32; 2],
    pub scale: [f32; 2],
}

impl TriangleUniform {
    pub fn new(color: [f32; 4], resolution: [f32; 2]) -> Self {
        Self {
            color,
            resolution,
            translation: [0., 0.],
            rotation: [1., 0.],
            scale: [1.0, 1.0],
        }
    }
}
