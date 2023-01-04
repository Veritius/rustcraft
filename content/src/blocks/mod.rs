use bevy::prelude::Color;
use block_mesh::VoxelVisibility;
use rustcraft_modlib::block::traits::BlockDefinition;

pub struct Dirt;
impl BlockDefinition for Dirt {
    fn new() -> Self where Self: Sized { Self {} }
    fn id(&self) -> &'static str { "vanilla_dirt" }
    fn name(&self) -> &'static str { "Dirt" }
    fn color(&self) -> Color { Color::BEIGE }
    fn visibility(&self) -> VoxelVisibility { VoxelVisibility::Opaque }
}

pub struct Stone;
impl BlockDefinition for Stone {
    fn new() -> Self where Self: Sized { Self {} }
    fn id(&self) -> &'static str { "vanilla_stone" }
    fn name(&self) -> &'static str { "Stone" }
    fn color(&self) -> Color { Color::GRAY }
    fn visibility(&self) -> VoxelVisibility { VoxelVisibility::Opaque }
}