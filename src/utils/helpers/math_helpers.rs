pub fn translation_3x3(translation: [f32; 2]) -> cgmath::Matrix3<f32> {
    cgmath::Matrix3 {
        x: [1.0, 0.0, 0.0,].into(),
        y: [0.0, 1.0, 0.0,].into(),
        z: [translation[0], translation[1], 1.0,].into(),
    }
}

pub fn rotation_3x3(angle: f32) -> cgmath::Matrix3<f32> {
    let c = angle.cos();
    let s = angle.sin();

    cgmath::Matrix3 {
        x: [c, s, 0.0,].into(),
        y: [-s, c, 0.0,].into(),
        z: [0.0, 0.0, 1.0,].into(),
    }
}

pub fn scale_3x3(scale: [f32; 2]) -> cgmath::Matrix3<f32> {
    cgmath::Matrix3 {
        x: [scale[0], 0.0, 0.0,].into(),
        y: [0.0, scale[1], 0.0,].into(),
        z: [0.0, 0.0, 1.0,].into(),
    }
}