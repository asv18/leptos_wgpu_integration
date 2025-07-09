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

    // lags like crazy on my computer - look into why?
    fn gen_polygon(num_sides: u16, radius: f32) -> (Vec<PolygonVertex>, Vec<u16>) {
        let angle = std::f32::consts::PI * 2.0 / num_sides as f32;
        let vertices = (0..(num_sides * 3))
            .map(|i| {
                let theta = angle * i as f32;
                PolygonVertex {
                    position: [radius * theta.sin(), radius * theta.cos(), 0.0],
                    color: [radius * theta.sin(), radius * theta.cos(), 1.0],
                }
                // [(1.0 + theta.cos()) / 2.0, (1.0 + theta.sin()) / 2.0, 1.0]
            })
            .collect();

        let num_triangles = (num_sides * 3) - 2;
        let indices = (1u16..num_triangles + 1)
            .into_iter()
            .flat_map(|i| vec![0, i + 1, i])
            .collect();

        (vertices, indices)
    }

    fn get_position(&self) -> [f32; 3] {
        self.position
    }

    fn set_position(&mut self, new: [f32; 3]) {
        self.position = new;
    }
}
