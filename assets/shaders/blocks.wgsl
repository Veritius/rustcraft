// Shader for chunk meshes

struct FragmentInput {
    @location(0) repeat_count: uvec2,
}

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    
}