use std::{sync::{Arc, RwLock}, collections::BTreeMap, fs, path::PathBuf};
use bevy::{render::once_cell::sync::Lazy, prelude::{Handle, Image, Assets, AssetServer, Commands, Res}, math::Rect};
use serde::Deserialize;

pub(crate) static BLOCK_ATLAS_TEXTURE: Lazy<Arc<RwLock<Option<BlockAtlas>>>> = Lazy::new(||Arc::new(RwLock::new(None)));

pub(super) fn block_atlas_creation_system(
    asset_server: Res<AssetServer>,
) {
    let handle: Handle<Image> = asset_server.load("textures/blocks.png");
    let mut atlas = BlockAtlas { image: handle, map: BTreeMap::new() };

    let mut path = PathBuf::new();
    path.push("assets/textures/blocks.ron");
    let file = fs::read_to_string(path).expect("Should have been able to read the file");
    let parsed: BlockTextures = ron::from_str(&file).expect("Block texture file should have been valid");

    const BLOCK_WIDTH: usize = 16;
    const IMAGE_WIDTH: usize = 512;
    const BLOCK_FRAC: f32 = BLOCK_WIDTH as f32 / IMAGE_WIDTH as f32;

    fn magic(number: u16) -> f32 {
        number as f32 * BLOCK_FRAC
    }

    for texture in &parsed.0 {
        atlas.map.insert(
            texture.name.clone(),
            Rect::new(
                magic(texture.pos[0]), // top left
                magic(texture.pos[1]), // bottom left
                magic(texture.pos[0]) + BLOCK_FRAC, // top right
                magic(texture.pos[1]) + BLOCK_FRAC, // bottom right
            )
        );
    }

    *BLOCK_ATLAS_TEXTURE.write().unwrap() = Some(atlas);
}

#[derive(Debug)]
pub struct BlockAtlas{
    pub image: Handle<Image>,
    pub map: BTreeMap<String, Rect>
}

#[derive(Deserialize)]
struct BlockTextures(Vec<BlockTexture>);

#[derive(Deserialize)]
struct BlockTexture {
    name: String,
    pos: [u16;2],
}