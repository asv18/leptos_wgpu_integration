use crate::utils::graphics::types::buffers::Vertex;

pub mod state;
pub mod types;

pub fn create_circle_vertices(
    radius: f32,
    num_subdivisions: u16,
    inner_radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> (Vec<Vertex>, Vec<u16>) {
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

    (vertex_data, indices)
}
