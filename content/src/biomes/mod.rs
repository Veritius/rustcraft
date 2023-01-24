pub mod defs;

pub mod attributes {
    use rustcraft_modlib::{
        world::generation::biome::registry::BiomeAttribute,
        registries::attributes::AttributeKind,
    };

    /// The altitude range this biome should spawn in.
    pub const ATTRIBUTE_GENVAR_HEIGHT: BiomeAttribute =
        BiomeAttribute::new("biome_genvar_height", 1, AttributeKind::RangeI32);
    /// The temperature range this biome should spawn in.
    pub const ATTRIBUTE_GENVAR_TEMPERATURE: BiomeAttribute =
        BiomeAttribute::new("biome_genvar_temperature", 2, AttributeKind::RangeI32);
    /// The humidity range this biome should spawn in.
    pub const ATTRIBUTE_GENVAR_HUMIDITY: BiomeAttribute =
        BiomeAttribute::new("biome_genvar_humidity", 3, AttributeKind::RangeU16);
}