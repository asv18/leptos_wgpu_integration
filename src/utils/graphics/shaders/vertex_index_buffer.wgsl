// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    // @location(1) color: vec3<f32>,
};

struct ColorStruct {
    color: vec4f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> color_struct: ColorStruct;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = color_struct.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}