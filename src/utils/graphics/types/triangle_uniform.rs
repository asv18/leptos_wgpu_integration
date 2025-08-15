#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleUniform {
    color: [f32; 4],
    resolution: [f32; 2],
    _pad0: [f32; 2],
    pub matrix: [[f32; 3]; 3],
    _pad1: [f32; 3],
}

impl TriangleUniform {
    pub fn new(color: [f32; 4], resolution: [f32; 2]) -> Self {
        Self {
            color,
            resolution,
            _pad0: [0.0, 0.0],
            matrix: [
                [1.0, 0., 0.],
                [0., 1.0, 0.],
                [0., 0., 1.0],
            ],
            _pad1: [0.0, 0.0, 0.0],
        }
    }
}
