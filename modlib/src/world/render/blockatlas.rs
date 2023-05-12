use std::{sync::{Arc, RwLock}, collections::BTreeMap};
use bevy::{render::once_cell::sync::Lazy, prelude::{Handle, Image, Assets}, math::Rect};
use serde::{Serialize, Deserialize};

pub(crate) static BLOCK_ATLAS_TEXTURE: Lazy<Arc<RwLock<Option<BlockAtlas>>>> = Lazy::new(||Arc::new(RwLock::new(None)));

pub(super) fn block_atlas_creation_system(

) {

}

pub struct BlockAtlas(pub BTreeMap<String, Rect>);

#[derive(Serialize, Deserialize)]
struct BlockTextures(Vec<BlockTexture>);

#[derive(Serialize, Deserialize)]
struct BlockTexture {
    name: String,
    pos: [u16;2],
}