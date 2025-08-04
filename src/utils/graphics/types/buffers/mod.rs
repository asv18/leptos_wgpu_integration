#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleUniform {
    color: [f32; 4],
    scale: [f32; 2],
    offset: [f32; 2],
}

impl TriangleUniform {
    pub fn new(color: [f32; 4], scale: [f32; 2], offset: [f32; 2]) -> Self {
        Self {
            color,
            scale,
            offset,
        }
    }
}

pub struct TriangleListItem {
    uniform: TriangleUniform,
    uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl TriangleListItem {
    pub fn new(
        uniform: TriangleUniform,
        uniform_buffer: wgpu::Buffer,
        bind_group: wgpu::BindGroup,
    ) -> Self {
        Self {
            uniform,
            uniform_buffer,
            bind_group,
        }
    }
}
