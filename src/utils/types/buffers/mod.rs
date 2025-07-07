pub mod polygon_buffer;
pub mod polygon_vertex;

pub trait Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2];
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
