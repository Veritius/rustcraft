use bevy::{prelude::*, ecs::system::SystemParam};
use crate::debug::{DebugMenuOpen, AppendDebugMenuMessage};
use self::{
    chunk::{registry::{ChunkRegistry, ChunkCoordinate, ChunkState}, Chunk, meshing::BeingRemeshed},
    block::{
        registry::BlockRegistry,
        entity::BlockEntity, Block,
    },
};

pub mod block;
pub mod chunk;
pub mod generation;

#[derive(SystemParam)]
pub struct WorldMapHelpers<'w, 's> {
    pub block_registry: Res<'w, BlockRegistry>,
    pub chunk_registry: Res<'w, ChunkRegistry>,
    pub blocks: Query<'w, 's, (Entity, &'static BlockEntity)>,
    pub chunks: Query<'w, 's, (Entity, &'static Chunk)>,
}

impl WorldMapHelpers<'_, '_> {
    /// Gets a block from any coordinates in the world. Returns `Some` if the chunk is loaded, `None` if not.
    pub fn get_block(&self, pos: IVec3) -> Option<Block> {
        let chunk_offset = pos / 16;
        let block_offset = (pos + IVec3::splat(16)) % 16;

        let chunk = match self.chunk_registry.get(chunk_offset.into()) {
            ChunkState::Present(entity) => entity,
            _ => { return None }
        };

        let chunk = self.chunks.get(chunk).expect("Chunk was in registry but not in query!").1;
        Some(chunk.get_block(block_offset.x as usize, block_offset.y as usize, block_offset.z as usize).clone())
    }

    pub fn get_chunk_or_none(&self, coord: ChunkCoordinate) -> Option<&Chunk> {
        match self.chunk_registry.get(coord) {
            ChunkState::Present(entity) => {
                match self.chunks.get(entity) {
                    Ok(query_result) => Some(query_result.1),
                    Err(_) => None,
                }
            },
            _ => None,
        }
    }
}

pub(crate) fn debug_data_system(
    open: Option<Res<DebugMenuOpen>>,
    mut events: EventWriter<AppendDebugMenuMessage>,
    world_map: WorldMapHelpers,
    meshing_chunks: Query<(), With<BeingRemeshed>>,
) {
    if open.is_none() { return; }

    let mut all_chunks: u32 = 0;
    let mut mid_generation: u32 = 0;
    let mut present: u32 = 0;
    for (_, value) in world_map.chunk_registry.get_inner_registry().iter() {
        match value {
            ChunkState::BeingGenerated => { mid_generation += 1; },
            ChunkState::Present(_) => { present += 1; },
            _ => panic!("Variant Absent shouldn't have been in the chunk registry"),
        }
        all_chunks += 1;
    }

    let mut block_count: u16 = 0;
    for (_, _) in world_map.block_registry.get_inner_registry() { block_count += 1; }

    let mut meshing_chunks_count: u32 = 0;
    for _ in meshing_chunks.iter() { meshing_chunks_count += 1; }

    let mut text = String::new();
    text.push_str(&format!("Chunks (all/active/generating/remeshing): {}/{}/{}/{}\n", all_chunks, present, mid_generation, meshing_chunks_count));
    text.push_str(&format!("Generic block types defined: {}\n", block_count));

    events.send(AppendDebugMenuMessage::new(TextSection { value: text, style: Default::default() }));
}