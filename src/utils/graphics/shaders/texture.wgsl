struct VSOutput {
    @builtin(position) position: vec4f,
    @location(0) texcoord: vec2f, // texcoord is a vector used to pass texture coordinates into our fragment shader
};
 
@vertexfn vs_main(
    @builtin(vertex_index) vertexIndex: u32
) -> VSOutput {
    let pos = array<vec2f, 6>(
        // 1st triangle
    vec2f(0.0, 0.0),  // center
    vec2f(1.0, 0.0),  // right, center
    vec2f(0.0, 1.0),  // center, top

        // 2nd triangle
    vec2f(0.0, 1.0),  // center, top
    vec2f(1.0, 0.0),  // right, center
    vec2f(1.0, 1.0),  // right, top
);

    let vsOutput: VSOutput;
    let xy = pos[vertexIndex];
    vsOutput.position = vec4f(xy, 0.0, 1.0);
    vsOutput.texcoord = xy;
    return vsOutput;
}
 
@group(0) @binding(0) var ourSampler: sampler;
@group(0) @binding(1) var ourTexture: texture_2d<f32>;
 
@fragmentfn fs_main(in: VSOutput) -> @location(0) vec4f {
    return textureSample(ourTexture, ourSampler, in.texcoord);
}