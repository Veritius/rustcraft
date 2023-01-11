use bevy::prelude::Color;
use rustcraft_modlib::world::{
    block::traits::BlockDefinition,
    chunk::meshing::MeshingVisibility
};

pub struct Grass;
impl BlockDefinition for Grass {
    fn new() -> Self where Self: Sized { Self {} }
    fn str_id(&self) -> &'static str { "vanilla_grass" }
    fn name(&self) -> &'static str { "Grass" }
    fn color(&self) -> Color { Color::DARK_GREEN }
    fn visibility(&self) -> MeshingVisibility { MeshingVisibility::Opaque }
}

pub struct Dirt;
impl BlockDefinition for Dirt {
    fn new() -> Self where Self: Sized { Self {} }
    fn str_id(&self) -> &'static str { "vanilla_dirt" }
    fn name(&self) -> &'static str { "Dirt" }
    fn color(&self) -> Color { Color::hsl(0.24, 0.96, 0.1) }
    fn visibility(&self) -> MeshingVisibility { MeshingVisibility::Opaque }
}

pub struct Sand;
impl BlockDefinition for Sand {
    fn new() -> Self where Self: Sized { Self {} }
    fn str_id(&self) -> &'static str { "vanilla_sand" }
    fn name(&self) -> &'static str { "Sand" }
    fn color(&self) -> Color { Color::BEIGE }
    fn visibility(&self) -> MeshingVisibility { MeshingVisibility::Opaque }
}

pub struct Stone;
impl BlockDefinition for Stone {
    fn new() -> Self where Self: Sized { Self {} }
    fn str_id(&self) -> &'static str { "vanilla_stone" }
    fn name(&self) -> &'static str { "Stone" }
    fn color(&self) -> Color { Color::GRAY }
    fn visibility(&self) -> MeshingVisibility { MeshingVisibility::Opaque }
}