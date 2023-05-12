#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

@group(1) @binding(0)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(1)
var base_color_sampler: sampler;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) repeat: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) repeat: vec2<f32>,
};

#import bevy_pbr::mesh_functions

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.uv = vertex.uv;
    out.repeat = vertex.repeat;
    return out;
}

struct FragmentInput {
    @location(0) uv: vec2<f32>,
    @location(1) repeat: vec2<f32>,
}

fn grid(
    uv: vec2<f32>,
    columns: f32,
    rows: f32
) -> vec2<f32> {
    return fract(vec2(uv.x * columns, uv.y * rows));
}

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    let grid_uv: vec2<f32> = grid(input.uv, input.repeat.x, input.repeat.y);
    return textureSample(base_color_texture, base_color_sampler, grid_uv);
}