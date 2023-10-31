# Rustcraft
A Minecraft clone written in Rust using the Bevy game engine, prioritising performance, memory usage, and extensibility.

## Advantages
### Grids and chunks
Rustcraft supports multiple voxel grids within the same Bevy [World]. Simply put, a `Grid` is an entity that is a parent of `Chunk` entities. Grids can be translated, rotated, and scaled freely. They can even be dynamic physics objects, falling and colliding with everything else. You could even make a 3D version of the [Falling Sand engine] if you really wanted to.

Following the strategy a lot of voxel games use, including Minecraft, the voxel world is broken into `Chunk`s, 16x16x16 cubes of voxels used for bulk loading and unloading. Note that Rustcraft uses chunking vertically as well, meaning the maximum height is unlimited.

### Voxel registry
Rustcraft achieves complex voxel behaviors without using a lot of memory by using `BlockId`s. `BlockId`s act as a key used to access a map of 'attributes'. Attributes are added during `App` creation and stored in the `BlockRegistry`, which after starting, becomes immutable, allowing it to be used concurrently. Blocks can still have unique data using `Any` or even be a Bevy `Entity` without much overhead.

[World]: https://docs.rs/bevy/0.11.3/bevy/ecs/world/struct.World.html
[Falling Sand engine]: https://www.youtube.com/watch?v=prXuyMCgbTc
[Cubic Chunks]: https://github.com/OpenCubicChunks/CubicChunks