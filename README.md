# rustcraft
A little test of a Minecraft-like voxel world in Rust, using the [Bevy game engine](https://bevyengine.org/).
Mostly intended as an exercise in optimisation, not an actual full-fledged game.

## Features
### Asynchronous and parallel
All game logic is parallel thanks to Bevy's scheduler.

The really computationally expensive tasks, like meshing and procedural generation, are asynchronous (running across game ticks) thanks to [AsyncComputeTaskPool](https://docs.rs/bevy/0.9.0/bevy/tasks/struct.AsyncComputeTaskPool.html).
Technically, this breaks the Bevy design pattern, as there is a Rust `static` storing block information outside of the game engine. However, this is necessary to perform block-related async tasks without memory copies.

### Efficient resource usage
Blocks are stored as a 16-bit unsigned integer ID, that is used by game systems to point towards a 'registry' containing block information.
There is no pre-set block data, and it has to be appended to a map, similar to the way `wgpu` vertex attributes work.

Like Minecraft, blocks are stored in 'chunks', allowing fast loading and unloading. Each chunk takes up a few kilobytes in the default configuration (16x16x16).
The array also stores Bevy entity IDs, allowing more complicated blocks to be made using ECS components and processed normally. Best of both worlds, basically.

The greedy meshing algorithm is used to minimise the amount of triangles used to draw the world.
The `texturing` branch contains code that is meant to draw repeating textures to the triangles, but it's unfinished and very broken.

## Screenshots
![image](https://github.com/Veritius/rustcraft/assets/45957058/67eed822-10c4-4d41-8be9-efa076d5f2d0)
![image](https://github.com/Veritius/rustcraft/assets/45957058/c13cb96e-cd91-467a-aaa7-64413b3ab9d0)
