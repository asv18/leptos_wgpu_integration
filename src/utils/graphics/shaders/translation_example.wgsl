struct Uniforms {
    color: vec4f,
    resolution: vec2f,
    _pad: vec2f,
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

    let position = vert.position;

    // Normalize pixel coordinates to 0..1, origin top-left
    let zeroToOne = vert.position / uni.resolution;

    // Convert to clip space -1..1
    let clipSpace = zeroToOne * 2.0 - vec2f(1.0, 1.0);

    // Flip Y because origin is top-left in pixels but clip space is bottom-left
    let flippedClipSpace = vec2f(clipSpace.x, 0.5 * clipSpace.y);

    vsOut.position = vec4f(flippedClipSpace, 0.0, 1.0);
    return vsOut;
}

@fragment fn fs_main(vsOut: VSOutput) -> @location(0) vec4f {
    return uni.color;
}