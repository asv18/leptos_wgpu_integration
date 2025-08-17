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
    translation: [f32; 2],
    scale: [f32; 2],

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
            translation: [500.0, 200.0],
            scale: [1.0, 1.0],
        }
    }

    pub fn handle_key(
        &mut self,
        code: KeyCode,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        canvas_size: &PhysicalSize<u32>,
    ) -> wgpu::BindGroup {
        match code {
            KeyCode::KeyCodeArrowRight => {
                self.translation = [
                    (self.translation[0] + 5.0).min(canvas_size.width as f32 - 100.0),
                    self.translation[1],
                ];
            }
            KeyCode::KeyCodeArrowLeft => {
                self.translation = [(self.translation[0] - 5.0).max(0.0), self.translation[1]];
            }
            KeyCode::KeyCodeArrowUp => {
                self.translation = [self.translation[0], (self.translation[1] - 5.0).max(0.0)];
            }
            KeyCode::KeyCodeArrowDown => {
                self.translation = [
                    self.translation[0],
                    (self.translation[1] + 5.0).min(canvas_size.height as f32 - 150.0),
                ];
            }
            KeyCode::KeyCodeR => {
                self.rotation = (self.rotation - 0.1).max(-std::f32::consts::PI * 2.0);
            }
            KeyCode::KeyCodeQ => {
                self.rotation = (self.rotation + 0.1).min(std::f32::consts::PI * 2.0);
            }
            KeyCode::KeyCodeW => {
                self.scale[1] = (self.scale[1] + 0.2).min(5.0);
            }
            KeyCode::KeyCodeS => {
                self.scale[1] = (self.scale[1] - 0.2).max(-5.0);
            }
            KeyCode::KeyCodeA => {
                self.scale[0] = (self.scale[0] - 0.2).max(-5.0);
            }
            KeyCode::KeyCodeD => {
                self.scale[0] = (self.scale[0] + 0.2).min(5.0);
            }
            _ => {}
        };

        self.rotation = self.rotation;

        self.update_triangle_uniform(
            device,
            canvas_size,
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

    pub fn update_triangle_uniform(
        &mut self,
        device: &wgpu::Device,
        canvas_size: &PhysicalSize<u32>,
    ) {
        let projection_matrix = gen_projection_3x3(canvas_size);

        let translation_matrix = gen_translation_3x3(self.translation);
        let rotation_matrix = gen_rotation_3x3(self.rotation);
        let scale_matrix = gen_scale_3x3(self.scale);

        let move_origin_matrix = gen_translation_3x3([-50., -75.]);

        let new_matrix: cgmath::Matrix3<f32> = projection_matrix * translation_matrix * rotation_matrix * scale_matrix * move_origin_matrix;

        self.triangle_uniform.pad_matrix(new_matrix);

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
