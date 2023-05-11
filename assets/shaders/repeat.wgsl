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

struct Vertex {
    @location(0) uv: vec2<f32>,
    @location(1) repeat: vec2<f32>,
};

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @location(1) repeat: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex.uv;
    out.repeat = vertex.repeat;
    return out;
}

struct FragmentInput {
    @location(0) uv: vec2<f32>,
    @location(1) repeat: vec2<f32>,
}

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    let grid_uv: vec2<f32> = grid(input.uv, input.repeat.x, input.repeat.y);
    return textureSample(base_color_texture, base_color_sampler, grid_uv);
}