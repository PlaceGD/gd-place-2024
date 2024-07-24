
struct Globals {
    screen_size: vec2<f32>,
    onion_size: vec2<f32>,
    camera_pos: vec2<f32>,
    zoom_scale: f32,
};


struct VertexInput {
    @location(0) pos: vec2<f32>,
};
struct InstanceInput {
    @location(1) pos: vec2<f32>,
    @location(2) t_x: vec2<f32>,
    @location(3) t_y: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(5) img: u32,
    @location(6) uv_pos: vec2<f32>,
    @location(7) uv_size: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) img: u32,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let pos = mat2x2<f32>(instance.t_x, instance.t_y) * vertex.pos + instance.pos;

    out.pos = vec4<f32>(pos / globals.screen_size * 2.0, 0.0, 1.0);
    out.uv = (vec2(vertex.pos.x, 1.0 - vertex.pos.y) * instance.uv_size + instance.uv_pos) / globals.onion_size;
    out.color = instance.color;
    out.img = instance.img;

    return out;
}


@group(0) @binding(0) var<uniform> globals: Globals;

@group(1) @binding(0) var t_tex_array: texture_2d_array<f32>;
@group(1) @binding(1) var s_tex_array: sampler;


fn fs_color(in: VertexOutput) -> vec4<f32> {
    if in.img == 0u {
        return in.color;
    } else {
        return textureSampleLevel(t_tex_array, s_tex_array, in.uv, in.img - 1u, 0.0) * in.color;
    }
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return fs_color(in);
}
@fragment
fn fs_main_sq_alpha(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = fs_color(in);
    return vec4(color.rgb * color.a, color.a);
}