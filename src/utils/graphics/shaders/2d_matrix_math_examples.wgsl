struct Uniforms {
    color: vec4f,
    matrix: mat3x3f,
    _pad1: vec4f,
};

struct Vertex {
    @location(0) position: vec2f,
};

struct VSOutput {
    @builtin(position) position: vec4f,
};

@group(0) @binding(0) var<uniform> uni: Uniforms;

@vertex fn vs_main(vert: Vertex) -> VSOutput {
    var vsOut: VSOutput;

    let clipSpace = (uni.matrix * vec3f(vert.position, 1)).xy;
 
    vsOut.position = vec4f(clipSpace, 0.0, 1.0);
    return vsOut;
}

@fragment fn fs_main(vsOut: VSOutput) -> @location(0) vec4f {
    return uni.color;
}