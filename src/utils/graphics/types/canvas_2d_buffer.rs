use crate::utils::graphics::types::{triangle_uniform::TriangleUniform, vertex::Vertex};
use wgpu::util::DeviceExt;

#[allow(unused)]
pub struct Canvas2dBuffer {
    // handle uniforms
    pub triangle_buffer: wgpu::Buffer,
    triangle_uniform: TriangleUniform,

    // handle vertices
    pub vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,

    // handle indices
    pub index_buffer: wgpu::Buffer,
    indices: Vec<u16>,
    pub num_indices: u32,
    // handle instancing
    // nothing for now
}

impl Canvas2dBuffer {
    pub fn new(
        device: &wgpu::Device,
        triangle_uniform: TriangleUniform,
        vertices: Vec<Vertex>,
        indices: Vec<u16>,
    ) -> Self {
        // Create triangle uniform buffer
        let triangle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Uniform Buffer"),
            contents: bytemuck::cast_slice(&[triangle_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // Create index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        let num_indices = indices.len();

        Self {
            triangle_buffer,
            triangle_uniform,
            vertex_buffer,
            vertices,
            index_buffer,
            indices,
            num_indices: num_indices as u32,
        }
    }
}

/*
Buffer relevant items:

triangle_buffer: wgpu::Buffer,
triangle_uniforms: Vec<TriangleUniform>,
bind_group: wgpu::BindGroup,
vertex_buffer: wgpu::Buffer,
vertices: Vec<Vertex>,

index_buffer: wgpu::Buffer,
indices: Vec<u16>,
*/
