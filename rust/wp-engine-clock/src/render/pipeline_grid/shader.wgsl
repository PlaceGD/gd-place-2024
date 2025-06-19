
fn eucl_mod(a: f32, b: f32) -> f32 {
    let r = a % b;
    if r < 0.0 { return r + abs(b); } else { return r; }
}


struct Globals {
    screen_size: vec2<f32>,
    time: f32,
    grid_opacity: f32,
    camera_pos: vec2<f32>,
    zoom_scale: f32,
    grid_shift: vec2<f32>,
    padding: vec2<f32>,
};


struct VertexInput {
    @location(0) pos: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    // @location(0) opacity: f32,
    // @location(1) grid_shift: vec2<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4((vertex.pos * 2.0 - 1.0), 0.0, 1.0);
    // out.opacity = globals.grid_opacity;
    // out.grid_shift = globals.grid_shift;

    return out;
}


@group(0) @binding(0) var<uniform> globals: Globals;



fn draw_grid(
    pos: vec2<f32>,
    grid_size: vec2<f32>,
    thickness: f32,
) -> bool {

    var half_thickness: f32;
    half_thickness = thickness / 2.0;

    let gs = grid_size;

    if eucl_mod(pos.x + half_thickness, gs.x) < thickness || eucl_mod(pos.y + half_thickness, gs.y) < thickness {
        return true;
    }

    return false;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // let fade = in.opacity; // TODO: hardcoded?

    let pos = (((in.pos.xy - globals.screen_size / 2.0) * vec2(1.0, -1.0) + globals.camera_pos * globals.zoom_scale) / globals.zoom_scale) + globals.grid_shift;

    if draw_grid(pos, vec2(30.0, 30.0), 1.0 / globals.zoom_scale) {
        return vec4<f32>(0.0, 0.0, 0.0, globals.grid_opacity);
    }

    return vec4<f32>(0.0);
}