use std::{ops::Range, sync::{Arc, RwLock}};
use bevy::prelude::{Vec2, IVec3};
use dyn_clone::DynClone;

use super::registry::BiomeData;

/// Biome selection scorers are used to select a biome for each chunk by calculating a point score for each possible option.
/// The biome with the highest point score wins.
pub trait BiomeSelectionScorer: 'static + Send + Sync + DynClone {
    fn get_point_score_for_coordinates(&self, coordinates: IVec3, biome_data: &BiomeData) -> f64;
}
dyn_clone::clone_trait_object!(BiomeSelectionScorer);