use bevy_ecs::component::Component;

pub enum Voxel {
    SimpleVoxel(SimpleVoxel),
    RotatableVoxel(RotatableVoxel),
    EntityVoxel(EntityVoxel), //make this an entity ref?
}

pub struct SimpleVoxel {
    id: u16
}

pub struct RotatableVoxel {
    id: u16,
    rot: bool //figure out what type should be used for 6 directional rotation
}

#[derive(Component)]
pub struct EntityVoxel {
    id: u16
}