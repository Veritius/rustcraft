use bevy_ecs::component::Component;

enum Voxel {
    SimpleVoxel(SimpleVoxel),
    RotatableVoxel(RotatableVoxel),
    EntityVoxel(EntityVoxel), //make this an entity ref?
}

struct SimpleVoxel {
    id: u16
}

struct RotatableVoxel {
    id: u16,
    rot: bool //figure out what type should be used for 6 directional rotation
}

#[derive(Component)]
struct EntityVoxel {
    id: u16
}