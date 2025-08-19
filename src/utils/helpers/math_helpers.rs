use crate::utils::graphics::types::size::PhysicalSize;

pub fn gen_scale_4x4(scale: [f32; 3]) -> cgmath::Matrix4<f32> {
    cgmath::Matrix4 {
        x: [scale[0], 0.0, 0.0, 0.0,].into(),
        y: [0.0, scale[1], 0.0, 0.0,].into(),
        z: [0.0, 0.0, scale[2], 0.0,].into(),
        w: [0.0, 0.0, 0.0, 1.0,].into(),
    }
}

pub fn gen_projection_4x4(size: &PhysicalSize<u32>, depth: f32) -> cgmath::Matrix4<f32> {
    let w = size.width as f32;
    let h = size.height as f32;

    cgmath::Matrix4::new(
        2.0 / w, 0.0,       0.0, 0.0,
        0.0,    -2.0 / h,   0.0, 0.0,
        0.0,     0.0,      1.0 / depth, 0.0,
       -1.0,     1.0,       0.5, 1.0,
    )
}