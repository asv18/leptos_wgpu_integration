use cgmath::SquareMatrix;

use crate::utils::graphics::types::{
    keycode::KeyCode, size::PhysicalSize, uniforms::camera_uniform::CameraUniform,
};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

pub trait CameraControl {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32>;
    fn handle_key(&mut self, code: &KeyCode) -> CameraUniform;
}

pub struct OrthoGraphicCamera {
    width: u32,
    height: u32,
    depth: u32,
}

impl OrthoGraphicCamera {
    pub fn new(canvas_size: &PhysicalSize<u32>, depth: u32) -> Self {
        Self {
            width: canvas_size.width,
            height: canvas_size.height,
            depth: depth,
        }
    }
}

impl CameraControl for OrthoGraphicCamera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // let view = cgmath::Matrix4::look_at_lh(self.eye, self.target, self.up);
        let proj = cgmath::ortho(
            0.0,
            self.width as f32,
            0.0,
            self.height as f32,
            0.0,
            self.depth as f32,
        );

        OPENGL_TO_WGPU_MATRIX * proj
    }

    fn handle_key(&mut self, code: &KeyCode) -> CameraUniform {
        let matrix = cgmath::Matrix4::<f32>::identity();

        match code {
            _ => {}
        };

        CameraUniform::new(self.build_view_projection_matrix() * matrix)
    }
}
