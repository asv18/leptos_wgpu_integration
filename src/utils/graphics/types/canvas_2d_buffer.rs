use crate::utils::{graphics::types::{
    keycode::KeyCode, size::PhysicalSize, triangle_uniform::TriangleUniform, vertex::Vertex,
}, helpers::math_helpers::*};
use wgpu::util::DeviceExt;

#[allow(unused)]
pub struct Canvas2dBuffer {
    // handle uniforms
    pub triangle_buffer: wgpu::Buffer,
    triangle_uniform: TriangleUniform,
    rotation: f32,

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
            rotation: 0.,
        }
    }

    pub fn handle_key(
        &mut self,
        code: KeyCode,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        canvas_size: &PhysicalSize<u32>,
    ) -> wgpu::BindGroup {
        let mut new_translation = [self.triangle_uniform.matrix[0][2], self.triangle_uniform.matrix[1][2]];

        let mut new_rotation = self.rotation;

        let mut new_scale_x = (self.triangle_uniform.matrix[0][0].powf(2.) + self.triangle_uniform.matrix[1][0].powf(2.)).sqrt();
        let mut new_scale_y = (self.triangle_uniform.matrix[0][1].powf(2.) + self.triangle_uniform.matrix[1][1].powf(2.)).sqrt();

        match code {
            KeyCode::KeyCodeArrowRight => {
                new_translation = [
                    (new_translation[0] + 5.0).min(canvas_size.width as f32 - 100.0),
                    new_translation[1],
                ];
            }
            KeyCode::KeyCodeArrowLeft => {
                new_translation = [(new_translation[0] - 5.0).max(0.0), new_translation[1]];
            }
            KeyCode::KeyCodeArrowUp => {
                new_translation = [new_translation[0], (new_translation[1] - 5.0).max(0.0)];
            }
            KeyCode::KeyCodeArrowDown => {
                new_translation = [
                    new_translation[0],
                    (new_translation[1] + 5.0).min(canvas_size.height as f32 - 150.0),
                ];
            }
            KeyCode::KeyCodeR => {
                new_rotation = (new_rotation - 0.1).max(-std::f32::consts::PI * 2.0);
            }
            KeyCode::KeyCodeQ => {
                new_rotation = (new_rotation + 0.1).min(std::f32::consts::PI * 2.0);
            }
            KeyCode::KeyCodeS => {
                new_scale_y = (new_scale_y + 0.2).min(5.0);
            }
            KeyCode::KeyCodeW => {
                new_scale_y = (new_scale_y - 0.2).max(-5.0);
            }
            KeyCode::KeyCodeA => {
                new_scale_x = (new_scale_x - 0.2).min(5.0);
            }
            KeyCode::KeyCodeD => {
                new_scale_x = (new_scale_x + 0.2).max(-5.0);
            }
            _ => {}
        };

        self.rotation = new_rotation;

        self.update_triangle_uniform(
            device,
            new_translation,
            new_rotation,
            [new_scale_x, new_scale_y],
        );

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind group"),
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.triangle_buffer.as_entire_binding(),
            }],
        })
    }

    fn update_triangle_uniform(
        &mut self,
        device: &wgpu::Device,
        new_translation: [f32; 2],
        new_rotation: f32,
        new_scale: [f32; 2],
    ) {
        let translation_3x3 = translation_3x3(new_translation);
        let rotation_3x3 = rotation_3x3(new_rotation);
        let scale_3x3 = scale_3x3(new_scale);

        let new_matrix = translation_3x3 * rotation_3x3 * scale_3x3;

        self.triangle_uniform.matrix = new_matrix.into();

        // Create triangle uniform buffer
        let triangle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Uniform Buffer"),
            contents: bytemuck::cast_slice(&[self.triangle_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        self.triangle_buffer = triangle_buffer;
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
