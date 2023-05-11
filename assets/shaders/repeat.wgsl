@group(1) @binding(0)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(1)
var base_color_sampler: sampler;

fn grid(
    uv: vec2<f32>,
    columns: f32,
    rows: f32
) -> vec2<f32> {
    return fract(vec2(uv.x * columns, uv.y * rows));
}

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
    @location(4) repeat: vec2<f32>,
) -> @location(0) vec4<f32> {
    let grid_uv: vec2<f32> = grid(uv, repeat.x, repeat.y);
    return textureSample(base_color_texture, base_color_sampler, grid_uv);
}