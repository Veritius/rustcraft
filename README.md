# Rustcraft
A Minecraft clone written in Rust using the Bevy game engine, prioritising performance, memory usage, and extensibility.

## Advantages
### Unlimited build height
Chunks are loaded vertically as they are horizontally, similar to the [Cubic Chunks] mod for Minecraft.

### Multiple grids
Multiple voxel grids can exist within the same Bevy [World]. By storing them as entities, with chunks as child entities, grids and chunks can be transformed and processed in parallel. They can even be dynamic physics objects, colliding with everything else. You could even make a 3D version of the [Falling Sand engine]

### Smart voxels
**TODO**

### Content ID system
**TODO**

[World]: https://docs.rs/bevy/0.11.3/bevy/ecs/world/struct.World.html
[Cubic Chunks]: https://github.com/OpenCubicChunks/CubicChunks
[Falling Sand engine]: https://www.youtube.com/watch?v=prXuyMCgbTc