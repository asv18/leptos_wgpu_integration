#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleUniform {
    pub color: [f32; 4],
    // scale: [f32; 2],
    // offset: [f32; 2],
}

impl TriangleUniform {
    pub fn new(color: [f32; 4] /*, scale: [f32; 2], offset: [f32; 2]*/) -> Self {
        Self {
            color,
            // scale,
            // offset,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // define how wide the vertex is within our memory
            step_mode: wgpu::VertexStepMode::Vertex, // notify the pipeline whether each element of the array in the buffer represents per-vertex or per-instance data
            attributes: &Self::ATTRIBUTES,
        }
    }
}
