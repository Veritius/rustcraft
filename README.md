# rustcraft
A little test of a Minecraft-like voxel world in Rust, using the [Bevy game engine](https://bevyengine.org/).
Mostly intended as an exercise in optimisation, not an actual full-fledged game.

## Features
### Asynchronous and parallel
All game logic is parallel thanks to Bevy's scheduler.

The really computationally expensive tasks, like meshing and procedural generation, are asynchronous (running across game ticks) thanks to [AsyncComputeTaskPool](https://docs.rs/bevy/0.9.0/bevy/tasks/struct.AsyncComputeTaskPool.html).
Technically, this breaks the Bevy design pattern, as there is a Rust `static` storing block information outside of the game engine. However, this is necessary to perform block-related async tasks without memory copies.

### Efficient resource usage
#### Chunks and the block registry
The world is divided into 'chunks', very much like Minecraft. This allows fast unloading and loading of chunks, as well as the ability to only load a part of the world. Each chunk is 16x16x16 rather than 16x16x256, allowing infinite chunk height by loading chunks the same way vertically as they are horizontally.

One chunk is a 3 dimensional array storing either a 16-bit block ID or a 16-bit ID for a Bevy entity ID in an associative array in the chunk. This solution combines the best of both worlds, allowing objects with completely identical values to have their information stored in a single place in memory, rather than being duplicated thousands of times. As well as that, it also allows complex, unique, compositional ECS objects to be stored in the array, with little extra memory usage.

The 16-bit block ID is used to access a 'registry', storing the information of that block type in an [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)-ed associative array. The registry technically breaks the Bevy design pattern, as it is accessible across ticks, but is necessary for generation to be asynchronous.

#### Greedy meshing
The game uses the greedy meshing algorithm to find a minimal amount of triangles to display a chunk, while sacrificing as little performance as possible. This is significantly faster compared to the Minecraft approach, which simply creates 2 triangles per visible block face.

There are no textures on faces at the moment, but the `texturing` branch is where the work on implementing this is happening. It's currently very broken.

### Modding, kinda
A lot of the game systems are specifically coded to allow new functionality. This is intended to be used with dynamic libraries, but I can't figure them out.

## Screenshots
![image](https://github.com/Veritius/rustcraft/assets/45957058/67eed822-10c4-4d41-8be9-efa076d5f2d0)
![image](https://github.com/Veritius/rustcraft/assets/45957058/c13cb96e-cd91-467a-aaa7-64413b3ab9d0)
