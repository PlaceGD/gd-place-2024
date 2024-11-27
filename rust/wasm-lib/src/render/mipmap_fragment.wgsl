@group(0) @binding(0)
var my_sampler: sampler;

@group(0) @binding(1)
var my_texture: texture_2d<f32>;

@fragment
fn main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let color = textureSample(my_texture, my_sampler, uv);
    return color;
}
