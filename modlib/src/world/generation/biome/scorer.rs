use std::ops::Range;
use bevy::prelude::{Vec2, IVec3};
use dyn_clone::DynClone;
use super::table::BiomeData;

dyn_clone::clone_trait_object!(BiomeSelectionScorer);
pub trait BiomeSelectionScorer: 'static + Send + Sync + DynClone {
    fn get_point_score_for_coordinates(&self, coordinates: IVec3, biome_data: &BiomeData) -> f64;
}