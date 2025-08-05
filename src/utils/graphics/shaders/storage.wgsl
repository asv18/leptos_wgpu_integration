struct VertexShaderStruct {
    color: vec4f, // each block is several bytes; we will need to modify our buffer to hold the size and values of our variables
    scale: vec2f, // scaling for our triangle
    offset: vec2f,
};

struct Vertex {
    @location(0) position: vec2f,
};

struct VSOutput {
    @builtin(position) position: vec4f, // lets us access our colors since our fragment shader doesn't have access to the instance index
    @location(0) color: vec4f,
}

@group(0) @binding(0) var<storage, read> shaderStructs: array<VertexShaderStruct>; // our uniform variable; kind of like a global variable
@group(0) @binding(1) var<storage, read> pos: array<Vertex>;

@vertex fn vs_main(
    vert: Vertex,
    @builtin(instance_index) instanceIndex: u32 // lets us reference the index of an instance so that we can do the rendering within our shader code by getting our struct elements from our arrays of structs
) -> VSOutput {
    let shaderStruct = shaderStructs[instanceIndex]; // index our shader structs

    var vsOut: VSOutput;
    vsOut.position = vec4f(vert.position * shaderStruct.scale + shaderStruct.offset, 0.0, 1.0);
    vsOut.color = shaderStruct.color;
    return vsOut;
}

@fragment fn fs_main(vsOut: VSOutput) -> @location(0) vec4f {
    return vsOut.color; // return color from our uniform
}