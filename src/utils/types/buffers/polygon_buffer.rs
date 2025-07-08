use std::marker::PhantomData;

use wgpu::{util::DeviceExt, Device};

use crate::utils::types::{buffers::Vertex, size::PhysicalSize};

pub struct PolygonBuffer<T: bytemuck::Pod + bytemuck::Zeroable + Vertex> {
    // check macro kata to make stuff like this more readable
    pub vertex_buffer: wgpu::Buffer,
    vertices: Vec<T>,
    pub index_buffer: wgpu::Buffer,
    indices: Vec<u16>,
    pub num_indices: u32,
    _marker: PhantomData<T>,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable + Vertex> PolygonBuffer<T> {
    pub fn new(device: &Device, vertices: &[T], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = indices.len() as u32;

        Self {
            vertex_buffer,
            vertices: vertices.into(),
            index_buffer,
            indices: indices.into(),
            num_indices,
            _marker: PhantomData,
        }
    }

    pub fn polygon_from_sides(
        device: &Device,
        canvas_size: &PhysicalSize<u32>,
        num_sides: u16,
        radius: f32,
    ) -> Self {
        let (vertices, indices) = T::gen_polygon(num_sides, radius, canvas_size);

        Self::new(device, &vertices, &indices)
    }
}
