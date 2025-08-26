struct TriangleUniform {
    color: vec4f,
};

struct CameraUniform {
    matrix: mat4x4f,
}

struct Vertex {
    @location(0) position: vec4f,
};

struct VSOutput {
    @builtin(position) position: vec4f,
};

@group(0) @binding(0) var<uniform> tri: TriangleUniform;
@group(0) @binding(1) var<uniform> camera: CameraUniform;

@vertex fn vs_main(vert: Vertex) -> VSOutput {
    var vsOut: VSOutput;

    vsOut.position = camera.matrix * vert.position;
    return vsOut;
}

@fragment fn fs_main(vsOut: VSOutput) -> @location(0) vec4f {
    return tri.color;
}