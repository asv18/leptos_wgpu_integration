use crate::utils::{
    graphics::types::{
        keycode::KeyCode, size::PhysicalSize, triangle_uniform::TriangleUniform, vertex::Vertex,
    },
    helpers::math_helpers::*,
};
use wgpu::util::DeviceExt;

#[allow(unused)]
pub struct Canvas2dBuffer {
    // handle uniforms
    pub triangle_buffer: wgpu::Buffer,
    triangle_uniform: TriangleUniform,

    rotation: [f32; 3],
    translation: [f32; 3],
    scale: [f32; 3],

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
            rotation: [0f32.to_radians(), 0f32.to_radians(), 0f32.to_radians()],
            translation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn handle_key(
        &mut self,
        code: &KeyCode,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        canvas_size: &PhysicalSize<u32>,
    ) -> wgpu::BindGroup {
        match code {
            // x-translation
            KeyCode::KeyCodeArrowRight => {
                self.translation[0] =
                    (self.translation[0] + 5.0).min(canvas_size.width as f32 - 100.0);
            }
            KeyCode::KeyCodeArrowLeft => {
                self.translation[0] = (self.translation[0] - 5.0).max(0.0);
            }
            // y-translation
            KeyCode::KeyCodeArrowUp => {
                self.translation[1] = (self.translation[1] - 5.0).max(0.0);
            }
            KeyCode::KeyCodeArrowDown => {
                self.translation[1] =
                    (self.translation[1] + 5.0).min(canvas_size.height as f32 - 150.0);
            }
            // TODO: z-translation
            KeyCode::KeyCodeSquareBracketLeft => {
                self.translation[2] = (self.translation[2] - 5.0).max(0.0);
            }
            KeyCode::KeyCodeSquareBracketRight => {
                self.translation[2] = (self.translation[2] + 5.0).min(400.0);
            }

            // x-rotation
            KeyCode::KeyCodeE => {
                self.rotation[0] = (self.rotation[0] - 0.1).max(-std::f32::consts::PI * 2.0);
            }
            KeyCode::KeyCodeQ => {
                self.rotation[0] = (self.rotation[0] + 0.1).min(std::f32::consts::PI * 2.0);
            }
            // y-rotation
            KeyCode::KeyCode1 => {
                self.rotation[1] = (self.rotation[1] - 0.1).max(-std::f32::consts::PI * 2.0);
            }
            KeyCode::KeyCode2 => {
                self.rotation[1] = (self.rotation[1] + 0.1).min(std::f32::consts::PI * 2.0);
            }
            // z-rotation
            KeyCode::KeyCodeZ => {
                self.rotation[2] = (self.rotation[2] - 0.1).max(-std::f32::consts::PI * 2.0);
            }
            KeyCode::KeyCodeX => {
                self.rotation[2] = (self.rotation[2] + 0.1).min(std::f32::consts::PI * 2.0);
            }
            // x-scaling
            KeyCode::KeyCodeW => {
                self.scale[1] = (self.scale[1] + 0.2).min(5.0);
            }
            KeyCode::KeyCodeS => {
                self.scale[1] = (self.scale[1] - 0.2).max(-5.0);
            }

            // y-scaling
            KeyCode::KeyCodeA => {
                self.scale[0] = (self.scale[0] - 0.2).max(-5.0);
            }
            KeyCode::KeyCodeD => {
                self.scale[0] = (self.scale[0] + 0.2).min(5.0);
            }

            // TODO: z-scaling
            _ => {}
        };

        self.update_triangle_uniform(queue, canvas_size);

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
        queue: &wgpu::Queue,
        canvas_size: &PhysicalSize<u32>,
    ) {
        // let projection_matrix = cgmath::ortho(
        //     0.0,
        //     canvas_size.width as f32,
        //     canvas_size.height as f32,
        //     0.0,
        //     0.0,
        //     400.,
        // );

        let projection_matrix = gen_projection_4x4(canvas_size, 400.);

        let model = cgmath::Matrix4::from_translation(self.translation.into())
            * cgmath::Matrix4::from_angle_x(cgmath::Rad(self.rotation[0]))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(self.rotation[1]))
            * cgmath::Matrix4::from_angle_z(cgmath::Rad(self.rotation[2]))
            * gen_scale_4x4(self.scale);

        let mvp = projection_matrix * model;

        self.triangle_uniform.matrix = mvp.into();

        // leptos::logging::log!("Model matrix: {model:?}\n\nProjection matrix: {projection_matrix:?}\n\nFinal matrix: {mvp:?}\n\nFinal matrix in raw form: {:?}", self.triangle_uniform.matrix);

        queue.write_buffer(
            &self.triangle_buffer,
            0,
            bytemuck::cast_slice(&[self.triangle_uniform]),
        )
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
