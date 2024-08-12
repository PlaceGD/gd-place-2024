

fn eucl_mod(a: f32, b: f32) -> f32 {
    let r = a % b;
    if r < 0.0 { return r + abs(b); } else { return r; }
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
    onion_size: vec2<f32>,
    camera_pos: vec2<f32>,
    zoom_scale: f32,
    time: f32,
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
    @location(3) uv_size: vec2<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let pos = mat2x2<f32>(instance.t_x, instance.t_y) * vertex.pos + instance.pos;

    out.pos = vec4<f32>(pos / globals.screen_size * 2.0, 0.0, 1.0);
    out.uv = (vec2(vertex.pos.x, 1.0 - vertex.pos.y) * instance.uv_size + instance.uv_pos);
    out.uv_size = instance.uv_size;
    out.color = instance.color;
    out.img = instance.img;

    switch instance.img {
        case 1u: {
            out.uv /= vec2<f32>(textureDimensions(t_1));
        }
        case 2u: {
            out.uv /= vec2<f32>(textureDimensions(t_2));
        }
        case 3u: {
            out.uv /= vec2<f32>(textureDimensions(t_3));
        }
        case 4u: {
            out.uv /= vec2<f32>(textureDimensions(t_4));
        }
        case 5u: {
            out.uv /= vec2<f32>(textureDimensions(t_5));
        }
        default: {}
    }

    return out;
}


@group(0) @binding(0) var<uniform> globals: Globals;

@group(1) @binding(0) var t_1: texture_2d<f32>;
@group(1) @binding(1) var s_1: sampler;
@group(1) @binding(2) var t_2: texture_2d<f32>;
@group(1) @binding(3) var s_2: sampler;
@group(1) @binding(4) var t_3: texture_2d<f32>;
@group(1) @binding(5) var s_3: sampler;
@group(1) @binding(6) var t_4: texture_2d<f32>;
@group(1) @binding(7) var s_4: sampler;
@group(1) @binding(8) var t_5: texture_2d<f32>;
@group(1) @binding(9) var s_5: sampler;

// fn zonky(id: u32) -> texture_2d<f32> {
//     switch id {
//         case 1u: {
//             return t_1;
//         }
//         case 2u: {
//             return t_2;
//         }
//         case 3u: {
//             return t_3;
//         }
//         case 4u: {
//             return t_4;
//         }
//         default: {
//             return t_5;
//         }
//     }
// }

fn fs_color(in: VertexOutput) -> vec4<f32> {

    switch in.img {
        case 0u: {
            return in.color;
        }
        case 1u: {
            return textureSampleLevel(t_1, s_1, in.uv, 0.0) * in.color;
        }
        case 2u: {
            return textureSampleLevel(t_2, s_2, in.uv, 0.0) * in.color;
        }
        case 3u: {
            return textureSampleLevel(t_3, s_3, in.uv, 0.0) * in.color;
        }
        case 4u: {
            return textureSampleLevel(t_4, s_4, in.uv, 0.0) * in.color;
        }
        case 5u: {
            return textureSampleLevel(t_5, s_5, in.uv, 0.0) * in.color;
        }
        case 1000u: {
            if !is_within_rect(in.uv, vec2(0.0), in.uv_size, -2.5) && eucl_mod(in.uv.x + in.uv.y - globals.time * 30.0, 10.0) < 5.0 {
                return in.color;
            } else {
                return vec4(0.0);
            }
        }
        default: {
            return vec4(sin(globals.time), cos(globals.time), 0.0, 1.0);
        }
    }

    // if in.img == 0u {
    // } else if in.img <= 100 {
    //     return textureSampleLevel(t_onion_linear, s_onion_linear, in.uv, in.img - 1u, 0.0) * in.color;
    // } else {
    //     return textureSampleLevel(t_onion_nearest, s_onion_nearest, in.uv, in.img - 101u, 0.0) * in.color;
    // }
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