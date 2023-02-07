use bevy::{render::{render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError, VertexFormat}, mesh::{MeshVertexBufferLayout, MeshVertexAttribute}}, reflect::TypeUuid, prelude::{Material, UVec2, Mesh}, pbr::{MaterialPipeline, MaterialPipelineKey}};

pub const ATTRIBUTE_TEXTURE_REPEAT_COUNT: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Repeat_Count", 5149634363, VertexFormat::Uint32x2);

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "3c8448b9-7748-473b-b374-441848c5a2f8"]
pub struct EfficientChunkMaterial {}

impl Material for EfficientChunkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/blocks.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/blocks.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_TEXTURE_REPEAT_COUNT.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}