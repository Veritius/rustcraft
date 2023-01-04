use bevy::prelude::Color;
use block_mesh::VoxelVisibility;
use super::traits::BlockDefinition;

pub struct Air;
impl BlockDefinition for Air {
    fn new() -> Self where Self: Sized { Self {} }
    fn id(&self) -> &'static str { "empty" }
    fn name(&self) -> &'static str { "Air" }
    fn color(&self) -> Color { Color::NONE }
    fn visibility(&self) -> VoxelVisibility { VoxelVisibility::Empty }
}