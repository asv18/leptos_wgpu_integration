// coordinate inter-stage variables between vertex shader and fragment shader
struct VertexShaderOutput {
    @builtin(position) position: vec4f, // this field is not an inter-stage variable, and is instead a builtin
    // @location(0) color: vec4f, // declare color at location 0
};

@vertex fn vs_main(
    @builtin(vertex_index) vertexIndex : u32
) -> VertexShaderOutput { // declare vertex shader and return type as our custom struct
    let pos = array(
        vec2f( 0.0,  0.5),  // top center
        vec2f(-0.5, -0.5),  // bottom left
        vec2f( 0.5, -0.5)   // bottom right
    );

    // // color array to pass to struct
    // var color = array<vec4f, 3>(
    //     vec4f(1, 0, 0, 1), // red
    //     vec4f(0, 1, 0, 1), // green
    //     vec4f(0, 0, 1, 1), // blue
    // );

    var vsOutput: VertexShaderOutput;

    vsOutput.position = vec4f(pos[vertexIndex], 0.0, 1.0);
    // vsOutput.color = color[vertexIndex];

    return vsOutput;
}

// return color declared in our struct for fragment shader
@fragment fn fs_main(
    fsInput: VertexShaderOutput
) -> @location(0) vec4f {
    // return fsInput.color;

    let red = vec4f(1, 0, 0, 1);
    let cyan = vec4f(0, 1, 1, 1);

    let grid = vec2u(fsInput.position.xy) / 8;
    let checker = (grid.x + grid.y) % 2 == 1;

    return select(red, cyan, checker);
}