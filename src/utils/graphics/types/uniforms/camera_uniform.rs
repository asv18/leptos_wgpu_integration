#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new(matrix: cgmath::Matrix4<f32>) -> Self {
        Self {
            matrix: matrix.into(),
        }
    }
}
