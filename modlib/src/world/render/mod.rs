pub mod shader;
pub mod blockatlas;

use bevy::prelude::{Plugin, App, MaterialPlugin};
use self::shader::RepeatingTextureMaterial;

pub struct BlockShadersPlugin;
impl Plugin for BlockShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<RepeatingTextureMaterial>::default());
    }
}