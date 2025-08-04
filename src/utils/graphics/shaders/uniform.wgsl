struct VertexShaderStruct {
    color: vec4f, // each block is several bytes; we will need to modify our buffer to hold the size and values of our variables
    scale: vec2f, // scaling for our triangle
    offset: vec2f,
};

struct VertexStruct {
    position: vec2f,
};

@group(0) @binding(0) var<uniform> shaderStruct: VertexShaderStruct; // our uniform variable; kind of like a global variable
@group(0) @binding(1) var<storage, read> pos: array<Vertex>;

struct VSOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

@vertex fn vs_main(
    @builtin(vertex_index) vertexIndex : u32
) -> @builtin(position) vec4f {
    return vec4f(pos[vertexIndex] * shaderStruct.scale + shaderStruct.offset, 0.0, 1.0); // use the uniform instead of some hard coded stuff
}

@fragment fn fs_main() -> @location(0) vec4f {
    return shaderStruct.color; // return color from our uniform
}