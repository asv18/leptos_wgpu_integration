#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleUniform {
    color: [f32; 4],
    pub matrix: [[f32; 4]; 3],
    _pad1: [f32; 4],
}

impl TriangleUniform {
    pub fn new(color: [f32; 4]) -> Self {
        Self {
            color,
            matrix: [
                [1.0, 0., 0., 0.0],
                [0., 1.0, 0., 0.0],
                [0., 0., 1.0, 0.0],
            ],
            _pad1: [0.0, 0.0, 0.0, 0.0],
        }
    }

    #[allow(unused)]
    pub fn clip_matrix(&self) -> [[f32; 3]; 3] {
        let mut mat: [[f32; 3]; 3] = [
            [0., 0., 0.],
            [0., 0., 0.],
            [0., 0., 0.],
        ];

        for i in 0..2 {
            for j in 0..2 {
                mat[i][j] = self.matrix[i][j];
            }
        }

        mat
    }

    pub fn pad_matrix(&mut self, matrix: cgmath::Matrix3<f32>) {
        for i in 0..=2 {
            for j in 0..=2 {
                self.matrix[i][j] = matrix[i][j];
            }
        }
    }
}
