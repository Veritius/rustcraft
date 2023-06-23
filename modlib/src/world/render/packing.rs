use std::{collections::BTreeMap, sync::{Arc, RwLock}};
use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::TextureFormatPixelInfo, once_cell::sync::Lazy}};
use rectangle_pack::{GroupedRectsToPlace, RectToInsert, pack_rects, TargetBin, RectanglePackError, RectanglePackOk};

pub static BLOCK_TEXTURE_ATLAS_DATA: Lazy<Arc<RwLock<BlockTextureAtlasData>>> = Lazy::new(|| Arc::new(RwLock::new(BlockTextureAtlasData::new(UVec2::splat(256)))));

pub struct BlockTextureAtlasData {
    atlas_handle: Option<Handle<Image>>,
    size: UVec2,
    handles: Vec<Handle<Image>>,
    rects: BTreeMap<Handle<Image>, (UVec2, UVec2)>,
}

impl BlockTextureAtlasData {
    fn new(size: UVec2) -> Self {
        Self {
            atlas_handle: None,
            size,
            handles: vec![],
            rects: BTreeMap::new(),
        }
    }

    /// Returns the handle for the atlas texture.
    fn handle(&self) -> Option<Handle<Image>> {
        self.atlas_handle.clone()
    }

    /// Adds the texture to the set used during packing.
    /// This does not repack by itself. Use `repack` for that.
    /// 
    /// The image passed should be of format `R8Unorm`, `Rg8Unorm`, or `Rgba8UnormSrgb`.
    /// If it's not, it will be ignored.
    fn add_texture(&mut self, handle: Handle<Image>) {
        self.handles.push(handle);
    }

    /// Finds the smallest image texture needed to pack all images, repacking the rectangles.
    /// 
    /// The `quality` is a value from 1-15, where 1 is faster, and 15 gets a smaller image.
    /// If the necessary dimensions are very large, running at a quality of 9 can take a very long time!
    /// 
    /// This will not be the smallest possible texture, but is close enough, for the sake of performance.
    fn minimise(&mut self, quality: u8, assets: &mut Assets<Image>) -> Result<(), TextureAtlasError> {
        if quality == 0 || quality > 15 { panic!("Packing quality was out of valid range at {quality}") }
        let increment = 512 - quality as u32 * 32;

        let old_size = self.size;
        self.size = UVec2::ZERO;
        if let Err(err) = self.repack(increment, assets) {
            self.size = old_size;
            Err(err)
        } else {
            Ok(())
        }
    }

    /// Repacks the rectangles, automatically expanding the texture atlas if necessary.
    fn repack(&mut self, expansion_increment: u32, assets: &mut Assets<Image>) -> Result<(), TextureAtlasError> {
        let mut rects: GroupedRectsToPlace<Handle<Image>, ()> = GroupedRectsToPlace::new();

        // Add all images to set
        for handle in &self.handles {
            // Get image value
            let image = match assets.get(&handle) { None => { warn!("Image could not be packed, asset inaccessible"); continue }, Some(v) => v };

            // Prevent incorrect formats being added
            match image.texture_descriptor.format {
                TextureFormat::R8Unorm |
                TextureFormat::Rg8Unorm |
                TextureFormat::Rgba8UnormSrgb => {
                    warn!("Image was the wrong format: {:?}", image.texture_descriptor.format);
                    continue;
                },
                _ => {}
            };

            // Push rect
            rects.push_rect(
                handle.clone(),
                None,
                RectToInsert::new(
                    image.texture_descriptor.size.width,
                    image.texture_descriptor.size.width,
                    1
                )
            );
        }

        let mut bins = BTreeMap::new();
        
        // Values related to automatically expanding the block atlas texture to fit everything.
        /// The image will not grow beyond this, for GPU reasons.
        const MAXIMUM_TEXTURE_SIZE: u32 = 15000;
        let mut expansion_value: u32 = 0;

        // Repeatedly try to expand to fit.
        let ret: RectanglePackOk<Handle<Image>, u8>;
        loop {
            // Set image size
            if !bins.is_empty() { bins.clear(); }
            bins.insert(1u8, TargetBin::new(self.size.x + expansion_value, self.size.y + expansion_value, 1));

            // Try to pack rectangles
            let placements = pack_rects(
                &rects,
                &mut bins,
                &rectangle_pack::volume_heuristic,
                &rectangle_pack::contains_smallest_box);
            
            match placements {
                Ok(val) => {
                    // Success, return and break.
                    ret = val;
                    break;
                },
                Err(err) => {
                    match err {
                        // Not enough size, try to expand
                        RectanglePackError::NotEnoughBinSpace => {
                            expansion_value += expansion_increment;
                            // Check we haven't gone over the limit
                            // This check works because it expands equally in both X and Y
                            if self.size.x + expansion_value > MAXIMUM_TEXTURE_SIZE { return Err(TextureAtlasError::ReachedTextureLimit) }
                            continue;
                        },
                    }
                },
            }
        }

        // Clear previous data and add new rectangles
        self.size += UVec2::splat(expansion_value);
        self.rects.clear();
        for pack in ret.packed_locations() {
            let pack_loc = pack.1.1;
            let start = UVec2::new(pack_loc.x(), pack_loc.y());
            let end = start + UVec2::new(pack_loc.width(), pack_loc.height());
            self.rects.insert(pack.0.clone(), (start, end));
        }

        // Texture to add atlas textures to.
        let mut atlas = Image::new_fill(
            Extent3d { width: self.size.x, height: self.size.y, depth_or_array_layers: 1 },
            TextureDimension::D2,
            &[255, 0, 255, 255], // bright pink
            TextureFormat::Rgba8UnormSrgb);
        let atlas_width = self.size.x as usize;
        let format_size = atlas.texture_descriptor.format.pixel_size();
        
        // Copy image data to atlas texture.
        for (image, (top_left, _bottom_right)) in &self.rects {
            // Get the sub-texture we'll be loading
            let image = assets
                .get(image)
                .expect("Handle should have been valid")
                .convert(TextureFormat::Rgba8UnormSrgb)
                .expect("Image should have been a convertable format");

            // This code is copied (with modifications) from here:
            // https://github.com/bevyengine/bevy/blob/70f91b2b9e50b86c54e8a1e566f6f61e186b5e9e/crates/bevy_sprite/src/texture_atlas_builder.rs#L100-L114
            let (rect_width, rect_height) = (image.texture_descriptor.size.width as usize, image.texture_descriptor.size.height as usize);
            let (rect_x, rect_y) = (top_left.x as usize, top_left.y as usize);

            // Write it to the atlas
            for (tex_y, bnd_y) in (rect_y..rect_y + rect_height).enumerate() {
                let begin = (bnd_y * atlas_width + rect_x) * format_size;
                let end = begin + rect_width * format_size;
                let tex_begin = tex_y * rect_width * format_size;
                let tex_end = tex_begin + rect_width * format_size;
                atlas.data[begin..end]
                    .copy_from_slice(&image.data[tex_begin..tex_end]);
            }
        }

        // Debugging thing to output the texture data
        // DONOTMERGE MERGEBLOCKER
        if atlas.clone().try_into_dynamic().unwrap().save("atlas_output.png").is_err() { error!("Failed to save the atlas texture."); };

        // Add to assets collection
        let atlas_handle = assets.add(atlas);
        self.atlas_handle = Some(atlas_handle);

        Ok(())
    }
}

pub enum TextureAtlasError {
    ReachedTextureLimit,
}