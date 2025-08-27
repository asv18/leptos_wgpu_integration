use crate::utils::graphics::types::{
    camera::CameraControl,
    keycode::KeyCode,
    uniforms::{camera_uniform::CameraUniform, triangle_uniform::TriangleUniform},
    vertex::Vertex,
};
use wgpu::util::DeviceExt;

#[allow(unused)]
pub struct Canvas2dBuffer<T: CameraControl> {
    // handle uniforms
    pub triangle_buffer: wgpu::Buffer,
    triangle_uniform: TriangleUniform,

    pub camera_buffer: wgpu::Buffer,
    camera_uniform: CameraUniform,
    pub camera: T,

    // rotation: [f32; 3],
    // translation: [f32; 3],
    // scale: [f32; 3],

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

impl<T: CameraControl> Canvas2dBuffer<T> {
    pub fn new(
        device: &wgpu::Device,
        triangle_uniform: TriangleUniform,
        vertices: Vec<Vertex>,
        indices: Vec<u16>,
        camera: T,
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

        let camera_uniform = CameraUniform::new(camera.build_view_projection_matrix());
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            triangle_buffer,
            triangle_uniform,
            vertex_buffer,
            vertices,
            index_buffer,
            indices,
            num_indices: num_indices as u32,
            camera_buffer,
            camera_uniform,
            camera,
        }
    }

    pub fn update_camera(&mut self, code: &KeyCode, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.camera_uniform = self.camera.handle_key(code);

        self.camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[self.camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        queue.write_buffer(
            &self.camera_buffer,
            1,
            bytemuck::cast_slice(&[self.camera_uniform]),
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
