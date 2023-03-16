// Shader for chunk meshes

#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) repeat: vec2<u32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, 
    @location(0) repeat: vec2<u32>,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.repeat = vertex.repeat;
    return out;
}

struct FragmentInput {
    @location(0) repeat: vec2<u32>,
}

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.4, 0.4, 0.8, 1.0);
}