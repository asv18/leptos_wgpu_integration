use crate::utils::types::{buffers::Vertex, size::PhysicalSize};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PolygonVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for PolygonVertex {
    // shorthand macro to do what is down in the commented out attributes code
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PolygonVertex>() as wgpu::BufferAddress, // define how wide the vertex is within our memory
            step_mode: wgpu::VertexStepMode::Vertex, // notify the pipeline whether each element of the array in the buffer represents per-vertex or per-instance data
            attributes: &Self::ATTRIBUTES,
            // attributes: &[
            //     // describe individual parts of the vertex - generally a 1:1 mapping with a struct's field, which is our case
            //     wgpu::VertexAttribute {
            //         offset: 0,          // defines the offset in bytes until the attribute starts
            //         shader_location: 0, // tells the shader where to store this attribute at i.e. `@location(0)` x: vec3<f32>
            //         format: wgpu::VertexFormat::Float32x3, // tells the shader the shape of the attribute
            //     },
            //     wgpu::VertexAttribute {
            //         offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            //         shader_location: 1,
            //         format: wgpu::VertexFormat::Float32x3,
            //     },
            // ],
        }
    }

    fn gen_polygon(
        num_sides: u16,
        radius: f32,
        canvas_size: &PhysicalSize<u32>,
    ) -> (Vec<PolygonVertex>, Vec<u16>) {
        let angle_step = std::f32::consts::TAU / num_sides as f32;

        let aspect_ratio = canvas_size.width as f32 / canvas_size.height as f32;

        // center vertex (for triangle fan)
        let mut vertices = vec![PolygonVertex {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0],
        }];

        // outer vertices
        for i in 0..=num_sides {
            let theta = i as f32 * angle_step;
            let x = radius * theta.cos();
            let y = radius * theta.sin();

            // Correct for aspect ratio: scale x-axis
            let corrected_x = x / aspect_ratio;

            vertices.push(PolygonVertex {
                position: [corrected_x, y, 0.0],
                color: [(1.0 + corrected_x) / 2.0, (1.0 + y) / 2.0, 1.0],
            });
        }

        // generate triangle fan indices
        let mut indices = Vec::new();
        for i in 1..=num_sides {
            indices.push(0); // center
            indices.push(i);
            indices.push(i + 1);
        }

        (vertices, indices)
    }
}
