
struct Globals {
    screen_size: vec2<f32>,
    onion_size: vec2<f32>,
    camera_pos: vec2<f32>,
    zoom_scale: f32,
};


struct VertexInput {
    @location(0) pos: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4(vertex.pos * 2.0 - 1.0, 0.0, 1.0);

    return out;
}


@group(0) @binding(0) var<uniform> globals: Globals;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = (in.pos.xy - globals.screen_size / 2.0) * vec2(1.0, -1.0);

    if length(pos + globals.camera_pos) < 30.0 {
        return vec4(1.0);
    }

    return vec4(0.0);
}