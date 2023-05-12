use std::{sync::{Arc, RwLock}, collections::BTreeMap};
use bevy::{render::once_cell::sync::Lazy, prelude::{Handle, Image, Assets}, math::Rect};

pub(crate) static BLOCK_ATLAS_TEXTURE: Lazy<Arc<Option<RwLock<DynamicAtlasTexture>>>> = Lazy::new(|| Arc::new(None));

/// Special implementation of texture atlases, similar to Bevy's, with some modifications.
/// 
/// - Data is accessible outside a tick.
///     - This is useful for asynchronous applications.
///     - Images can only be added to the atlas texture within a tick.
/// - Associated string rather than number
/// - Add textures at any point
///     - You can add textures at any point, and it'll repack.
///     - This will very likely break UVs.
pub struct DynamicAtlasTexture {
    pub handle: Handle<Image>,
    pub rects: BTreeMap<String, Rect>,
}

impl DynamicAtlasTexture {
    /// Adds textures to the atlas. This will always repack the image atlas, be careful!
    pub fn add_textures(&mut self, assets: &mut Assets<Image>, images: Vec<(String, Image)>) {
        // Take ownership of images for rect extraction.
        let mut images = images;

        // Get image asset associated with this atlas texture.
        let image_asset = assets.get_mut(&self.handle).expect("Atlas texture handle should have been valid!");

        // Extract existing rectangles.
        for rect in &self.rects {
            
        }

        // Delete everything in rects.
        self.rects.clear();
    }

    pub fn get_texture_uvs(&self, name: String) -> Rect {
        todo!();
    }
}