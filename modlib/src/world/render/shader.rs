use bevy::{
    render::{
        render_resource::{
            AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError, VertexFormat
        },
        mesh::{
            MeshVertexBufferLayout, MeshVertexAttribute
        }
    },
    reflect::TypeUuid,
    prelude::{
        Material, Mesh, Handle, Image, Color
    },
    pbr::{
        MaterialPipeline, MaterialPipelineKey
    }
};

pub const ATTRIBUTE_TEXTURE_REPEAT_COUNT: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Repeat_Count", 5149634363, VertexFormat::Float32x2);

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "3c8448b9-7748-473b-b374-441848c5a2f8"]
pub struct RepeatingTextureMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub atlas: Handle<Image>,
}

impl Material for RepeatingTextureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/repeat.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(3),
            ATTRIBUTE_TEXTURE_REPEAT_COUNT.at_shader_location(4),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}