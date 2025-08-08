// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    // @location(1) color: vec3<f32>,
};

struct ColorStruct {
    color: vec4f,
    scale: vec2f,
    offset: vec2f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0) var<storage, read> color_structs: array<ColorStruct>;

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(instance_index) instance_index: u32,
) -> VertexOutput {
    let color_struct = color_structs[instance_index];

    var out: VertexOutput;
    out.color = color_struct.color;
    out.clip_position = vec4<f32>(model.position[0] * color_struct.scale[0] + color_struct.offset[0], model.position[1] * color_struct.scale[1] + color_struct.offset[1], model.position[2], 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}