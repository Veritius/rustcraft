// Shader for chunk meshes

#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vertex(
    vertex: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0))
    return out;
}

@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.4, 0.4, 0.8, 1.0);
}