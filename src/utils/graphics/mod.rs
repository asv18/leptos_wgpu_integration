pub mod state;
pub mod types;

use rand::Rng;
use types::vertex::Vertex;

use crate::utils::graphics::types::{
    canvas_2d_buffer::Canvas2dBuffer, size::PhysicalSize, triangle_uniform::TriangleUniform,
};

#[allow(unused)]
pub fn create_circle_vertices(
    device: &wgpu::Device,
    canvas_size: &PhysicalSize<u32>,
    radius: f32,
    num_subdivisions: u16,
    inner_radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> Canvas2dBuffer {
    // 2 triangles per subdivision, 3 verts per tri, 2 values (xy) each.
    // let num_vertices = num_subdivisions * 3 * 2;
    let mut indices = Vec::with_capacity((num_subdivisions * 6) as usize);
    let mut vertex_data = Vec::with_capacity((num_subdivisions * 2 * 3 * 2) as usize);

    // 2 vertices per subdivision
    //
    // 0--1 4
    // | / /|
    // |/ / |
    // 2 3--5
    for i in 0..num_subdivisions {
        let base = i * 6;

        let angle1 =
            start_angle + (i as f32 + 0.0) * (end_angle - start_angle) / num_subdivisions as f32;
        let angle2 =
            start_angle + (i as f32 + 1.0) * (end_angle - start_angle) / num_subdivisions as f32;

        let c1 = angle1.cos();
        let s1 = angle1.sin();
        let c2 = angle2.cos();
        let s2 = angle2.sin();

        // first triangle
        vertex_data.push(Vertex {
            position: [c1 * radius, s1 * radius],
        });
        vertex_data.push(Vertex {
            position: [c2 * radius, s2 * radius],
        });
        vertex_data.push(Vertex {
            position: [c1 * inner_radius, s1 * inner_radius],
        });

        indices.push(base + 0);
        indices.push(base + 1);
        indices.push(base + 2);

        // second triangle
        vertex_data.push(Vertex {
            position: [c1 * inner_radius, s1 * inner_radius],
        });
        vertex_data.push(Vertex {
            position: [c2 * radius, s2 * radius],
        });
        vertex_data.push(Vertex {
            position: [c2 * inner_radius, s2 * inner_radius],
        });

        indices.push(base + 3);
        indices.push(base + 4);
        indices.push(base + 5);
    }

    let mut rng = rand::thread_rng();
    let triangle_uniform = TriangleUniform::new(
        [
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            rng.gen_range(0.0..=1.0),
            1.0,
        ],
    );

    leptos::logging::log!("{:?}", vertex_data);

    Canvas2dBuffer::new(device, triangle_uniform, vertex_data, indices)
}

#[allow(non_snake_case)]
#[rustfmt::skip]
pub fn create_F_buffer(
    device: &wgpu::Device,
    canvas_size: &PhysicalSize<u32>,
) -> Canvas2dBuffer {
    let vertex_data = [
        // left column
        0., 0.,
        30., 0.,
        0., 150.,
        30., 150.,
    
        // top rung
        30., 0.,
        100., 0.,
        30., 30.,
        100., 30.,
    
        // middle rung
        30., 60.,
        70., 60.,
        30., 90.,
        70., 90.,
    ];

    let index_data = [
        0 as u16,  1,  2,    2,  1,  3,  // left column
        4,  5,  6,    6,  5,  7,  // top run
        8,  9, 10,   10,  9, 11,  // middle run
    ];

    let mut vertices = Vec::with_capacity(vertex_data.len() / 2);

    for i in (0..vertex_data.len()).step_by(2) {
        vertices.push(Vertex {
            position: [vertex_data[i], vertex_data[i + 1]]
        });
    }

    let indices = index_data.to_vec();

    let mut rng = rand::thread_rng();
    let triangle_uniform = TriangleUniform::new([rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0), 1.0]);
    
    let mut buffer = Canvas2dBuffer::new(device, triangle_uniform, vertices, indices);

    buffer.update_triangle_uniform(device, canvas_size);

    buffer
}
