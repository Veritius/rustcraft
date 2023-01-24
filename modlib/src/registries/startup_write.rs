use std::sync::Arc;
use bevy::prelude::{Resource, App, StartupStage, Commands, Res};

/// Something that needs to be written at (Bevy) startup and then never written again, but read across threads.
/// 
/// This specific trait is for use as a Bevy resource, and can return an `Arc` of its internal data.
pub trait StartupWriteResource<B: StartupWriteBuffer<I>, I: StartupWriteInternal>: Resource {
    fn from_buffer(buffer: B) -> Self;
    fn return_internal_arc(&self) -> &Arc<I>;
}

/// Something that needs to be written at (Bevy) startup and then never written again, but read across threads.
///
/// This specific trait is used when the data is first being written at startup. It is later discarded.
pub trait StartupWriteBuffer<I: StartupWriteInternal>: Resource + Clone {
    fn new() -> Self;
}

/// Something that needs to be written at (Bevy) startup and then never written again, but read across threads.
///
/// This specific trait is the actual data in question. It will never be changed after first creation, and will be accessible via an `Arc`.
pub trait StartupWriteInternal: Clone {
    fn new() -> Self;
}

trait StartupWriteFns {
    fn add_startup_write_type<R: StartupWriteResource<B, I>, B: StartupWriteBuffer<I>, I: StartupWriteInternal>(&mut self) -> &mut Self;
}

impl StartupWriteFns for App {
    fn add_startup_write_type<R: StartupWriteResource<B, I>, B: StartupWriteBuffer<I>, I: StartupWriteInternal>(&mut self) -> &mut Self {
        self.insert_resource(B::new());

        self.add_startup_system_to_stage(StartupStage::PostStartup, |
            mut commands: Commands,
            buffer: Res<B>
        | {
            commands.insert_resource(R::from_buffer(buffer.clone()));
            commands.remove_resource::<B>();
        });

        self
    }
}