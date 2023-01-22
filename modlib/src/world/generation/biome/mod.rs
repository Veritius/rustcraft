use std::{
    collections::BTreeMap,
    ops::Range,
};
use bevy::{
    prelude::*,
    utils::HashMap,
};
use self::table::{BiomeData, BiomeTable};

pub mod considerations;
pub mod table;

pub trait AddBiome {
   fn add_biome(&mut self, biome: BiomeData) -> &mut Self;
}

impl AddBiome for App {
    fn add_biome(&mut self, biome: BiomeData) -> &mut Self {
        self.add_startup_system(move |mut biome_table: ResMut<BiomeTable>| {
            biome_table.add_biome_type(biome.clone());
        });

        self
    }
}

type BiomeId = u32;