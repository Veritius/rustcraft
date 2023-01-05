use bevy::prelude::Color;
use rustcraft_modlib::{
    block::traits::BlockDefinition,
    chunk::meshing::MeshingVisibility
};

pub struct Dirt;
impl BlockDefinition for Dirt {
    fn new() -> Self where Self: Sized { Self {} }
    fn id(&self) -> &'static str { "vanilla_dirt" }
    fn name(&self) -> &'static str { "Dirt" }
    fn color(&self) -> Color { Color::BEIGE }
    fn visibility(&self) -> MeshingVisibility { MeshingVisibility::Opaque }
}

pub struct Stone;
impl BlockDefinition for Stone {
    fn new() -> Self where Self: Sized { Self {} }
    fn id(&self) -> &'static str { "vanilla_stone" }
    fn name(&self) -> &'static str { "Stone" }
    fn color(&self) -> Color { Color::GRAY }
    fn visibility(&self) -> MeshingVisibility { MeshingVisibility::Opaque }
}