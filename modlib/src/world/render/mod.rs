pub mod shader;

use bevy::prelude::{Plugin, App, MaterialPlugin};
use self::shader::EfficientChunkMaterial;

pub struct BlockShadersPlugin;
impl Plugin for BlockShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<EfficientChunkMaterial>::default());
    }
}