use crate::utils::types::size::PhysicalSize;

pub mod polygon_buffer;
pub mod polygon_vertex;

pub trait Vertex: Sized {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2];
    fn desc() -> wgpu::VertexBufferLayout<'static>;
    fn gen_polygon(sides: u16, radius: f32) -> (Vec<Self>, Vec<u16>);

    fn get_position(&self) -> [f32; 3];
    fn set_position(&mut self, new: [f32; 3]);
}

pub enum VertexType {
    Colored,
    Texed,
}

// use polygon_vertex::PolygonVertex;
// Vertices are arranged in a counter-clockwise order: top, bottom left, bottom right, etc.
// pub const VERTICES: &[PolygonVertex] = &[
//     PolygonVertex {
//         position: [-0.0868241, 0.49240386, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // A
//     PolygonVertex {
//         position: [-0.49513406, 0.06958647, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // B
//     PolygonVertex {
//         position: [0.44147372, 0.2347359, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // E
//     PolygonVertex {
//         position: [-0.49513406, 0.06958647, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // B
//     PolygonVertex {
//         position: [-0.21918549, -0.44939706, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // C
//     PolygonVertex {
//         position: [0.44147372, 0.2347359, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // E
//     PolygonVertex {
//         position: [-0.21918549, -0.44939706, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // C
//     PolygonVertex {
//         position: [0.35966998, -0.3473291, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // D
//     PolygonVertex {
//         position: [0.44147372, 0.2347359, 0.0],
//         color: [0.5, 0.0, 0.5],
//     }, // E
// ];

// const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

// pub const NUM_VERTICES: u32 = VERTICES.len() as u32;
