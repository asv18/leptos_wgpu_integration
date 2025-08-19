pub mod state;
pub mod types;

use rand::Rng;
use types::vertex::Vertex;

use crate::utils::graphics::types::{
    canvas_2d_buffer::Canvas2dBuffer, size::PhysicalSize, triangle_uniform::TriangleUniform,
};

#[allow(non_snake_case)]
#[rustfmt::skip]
pub fn create_F_buffer(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    canvas_size: &PhysicalSize<u32>,
) -> Canvas2dBuffer {
    let vertex_data = [
        // left column
        0, 0, 0,
        30, 0, 0,
        0, 150, 0,
        30, 150, 0,
    
        // top rung
        30, 0, 0,
        100, 0, 0,
        30, 30, 0,
        100, 30, 0,
    
        // middle rung
        30, 60, 0,
        70, 60, 0,
        30, 90, 0,
        70, 90, 0,
    ];

    let index_data = [
        0 as u16,  1,  2,    2,  1,  3,  // left column
        4,  5,  6,    6,  5,  7,  // top run
        8,  9, 10,   10,  9, 11,  // middle run
    ];

    let mut vertices = Vec::with_capacity(vertex_data.len() / 2);

    for i in (0..vertex_data.len()).step_by(3) {
        vertices.push(Vertex {
            position: [vertex_data[i] as f32, vertex_data[i + 1] as f32, vertex_data[i + 2] as f32, 1.0]
        });
    }

    let indices = index_data.to_vec();

    let mut rng = rand::thread_rng();
    let triangle_uniform = TriangleUniform::new([rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0), 1.0]);
    
    let mut buffer = Canvas2dBuffer::new(device, triangle_uniform, vertices, indices);

    buffer.update_triangle_uniform(queue, canvas_size);

    buffer
}
