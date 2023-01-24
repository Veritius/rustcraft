//! Registries.

pub mod attributes;
pub mod startup_write;

use startup_write::{StartupWriteResource, StartupWriteInternal, StartupWriteBuffer};

pub trait Registry<B: RegistryBuffer<I, E, Id>, I: RegistryInternal<E, Id>, E: RegistryEntry, Id: RegistryIdentifier>: StartupWriteResource<B, I> {
    fn get_entry(&self) -> &E;
}

pub trait RegistryBuffer<I: RegistryInternal<E, Id>, E: RegistryEntry, Id: RegistryIdentifier>: StartupWriteBuffer<I> {
    fn add_entry(&mut self, entry: E);
}

pub trait RegistryInternal<E: RegistryEntry, Id: RegistryIdentifier>: StartupWriteInternal {
    fn get_entry(&self, id: Id) -> &E;
}

pub trait RegistryEntry {
    
}

pub trait RegistryIdentifier: PartialEq + Eq {

}