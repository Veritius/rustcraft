use bevy::{prelude::Mesh, render::mesh::PrimitiveTopology};
use super::chunk::Chunk;

fn generate_visual_mesh(chunk: &Chunk) {
    let ent = chunk.entity;
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
}