
fn eucl_mod(a: f32, b: f32) -> f32 {
    let r = a % b;
    if r < 0.0 { return r + abs(b); } else { return r; }
}
fn map(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    return (x - a) / (b - a) * (d - c) + c;
}
fn is_within_rect(
    pos: vec2<f32>,
    min: vec2<f32>,
    max: vec2<f32>,
    extend: f32,
) -> bool {
    return pos.x >= min.x - extend && pos.y >= min.y - extend && pos.x <= max.x + extend && pos.y <= max.y + extend;
}


struct Globals {
    screen_size: vec2<f32>,
    quality: f32,
    _unused: f32,
    camera_pos: vec2<f32>,
    zoom_scale: f32,
    time: f32,
    // end_anim_time: f32,
    // padding: array<f32, 2>,
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
    out.pos = vec4((vertex.pos * 2.0 - 1.0), 0.0, 1.0);

    return out;
}


@group(0) @binding(0) var<uniform> globals: Globals;


/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
/// MAKE SURE YOU CHCNAGE THIS WHEVERRE  IT IS USED
const LEVEL_WIDTH_BLOCKS: u32 = 800;
const LEVEL_HEIGHT_BLOCKS: u32 = 800;
const LEVEL_WIDTH_UNITS: u32 = LEVEL_WIDTH_BLOCKS * 30;
const LEVEL_HEIGHT_UNITS: u32 = LEVEL_HEIGHT_BLOCKS * 30;

const LEVEL_SIZE_VEC: vec2<f32> = vec2(f32(LEVEL_WIDTH_UNITS), f32(LEVEL_HEIGHT_UNITS));


fn draw_grid(
    pos: vec2<f32>,
    grid_size: vec2<f32>,
    thickness: f32,
) -> bool {

    // let thickness = thickness0 / globals.zoom_scale;

    var half_thickness: f32;
    // if thickness % 2.0 == 0.0 {
    half_thickness = thickness / 2.0;
    // } else {
    //     half_thickness = (thickness - 1.0) / 2.0;
    // };

    let gs = grid_size;

    if eucl_mod(pos.x + half_thickness, gs.x) < thickness || eucl_mod(pos.y + half_thickness, gs.y) < thickness {
        return true;
    }

    return false;
}

fn ease_out_expo(t: f32) -> f32 {
    return 1.0 - pow(2.0, -10.0 * t);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let fade = map(log2(globals.zoom_scale) * 12.0, -8.0, 24.0, 0.0, 1.0);


    let pos = ((in.pos.xy - globals.screen_size / 2.0) * vec2(1.0, -1.0) / globals.quality + globals.camera_pos * globals.zoom_scale) / globals.zoom_scale;
    // if length(pos) < 10.0 {
    //     return vec4<f32>(0.0, 1.0, 0.0, 1.0);
    // }
    var anim_val = 0.0;
    if globals.time < 0.0 {
        
        let end_trans01 = -globals.time;
        let end_anim_time = end_trans01 * 10.0;
        let dist_from_center = length(pos - globals.camera_pos);
        let delay = dist_from_center * 0.001;
        let explosion_d =
            ease_out_expo(min(max(((end_anim_time - delay) / 3.0),0.0),1.0));

        anim_val += explosion_d;
    }

    if is_within_rect(pos, vec2(0.0), LEVEL_SIZE_VEC, 2.0 / globals.zoom_scale) {
        if draw_grid(pos, LEVEL_SIZE_VEC, 4.0 / globals.zoom_scale / globals.quality) {
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
    }

    if is_within_rect(pos, vec2(0.0), LEVEL_SIZE_VEC, 0.5 / globals.zoom_scale) {
        if draw_grid(pos, vec2(30.0, 30.0), 1.0 / globals.zoom_scale / globals.quality) {
            return vec4<f32>(0.0, 0.0, 0.0, fade + anim_val);
        }
    }



    return vec4<f32>(0.0, 0.0, 0.0, anim_val);
}