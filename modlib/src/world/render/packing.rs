use std::collections::BTreeMap;

use bevy::{prelude::*, render::{render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::TextureFormatPixelInfo}};
use rectangle_pack::{GroupedRectsToPlace, RectToInsert, pack_rects, TargetBin, RectanglePackError, RectanglePackOk};

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

    /// Adds the texture to the set used during packing.
    /// This does not repack by itself. Use `repack` for that.
    fn add_texture(&mut self, handle: Handle<Image>) {
        self.handles.push(handle);
    }

    /// Finds the smallest image texture needed to pack all images, repacking the rectangles.
    /// This can be a very expensive operation.
    /// 
    /// This will not be the smallest possible texture, but is close enough, for the sake of performance.
    fn minimise(&mut self, assets: &mut Assets<Image>) -> Result<(), TextureAtlasError> {
        let old_size = self.size;
        self.size = UVec2::ZERO;
        if let Err(err) = self.repack(assets) {
            self.size = old_size;
            Err(err)
        } else {
            Ok(())
        }
    }

    /// Repacks the rectangles, automatically expanding the texture atlas if necessary.
    fn repack(&mut self, assets: &mut Assets<Image>) -> Result<(), TextureAtlasError> {
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
        /// The image size expands by EXPAND_AMOUNT pixels every try.
        /// Higher values will execute faster, but there'll be more unused space.
        /// Lower values will have a smaller image, but will take longer to run.
        const EXPAND_AMOUNT: u32 = 64;
        let mut expansion: u32 = 0;

        // Repeatedly try to expand to fit.
        let ret: RectanglePackOk<Handle<Image>, u8>;
        loop {
            // Set image size
            if !bins.is_empty() { bins.clear(); }
            bins.insert(1u8, TargetBin::new(self.size.x + expansion, self.size.y + expansion, 1));

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
                            expansion += EXPAND_AMOUNT;
                            // Check we haven't gone over the limit
                            // This check works because it expands equally in both X and Y
                            if self.size.x + expansion > MAXIMUM_TEXTURE_SIZE { return Err(TextureAtlasError::ReachedTextureLimit) }
                            continue;
                        },
                    }
                },
            }
        }

        // Clear previous data and add new rectangles
        self.size += UVec2::splat(expansion);
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