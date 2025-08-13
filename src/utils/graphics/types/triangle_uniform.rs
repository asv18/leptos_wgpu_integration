#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleUniform {
    color: [f32; 4],
    resolution: [f32; 2],
    _pad: [f32; 2],
}

impl TriangleUniform {
    pub fn new(color: [f32; 4], resolution: [f32; 2]) -> Self {
        Self {
            color,
            resolution,
            _pad: [0., 0.],
        }
    }
}
