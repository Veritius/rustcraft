//! Registries.

pub mod attributes;
pub mod startup_write;

use startup_write::{StartupWriteResource, StartupWriteInternal, StartupWriteBuffer};

pub trait Registry<B: RegistryBuffer<I, E>, I: RegistryInternal<E>, E: RegistryEntry>: StartupWriteResource<B, I> {

}

pub trait RegistryBuffer<I: RegistryInternal<E>, E: RegistryEntry>: StartupWriteBuffer<I> {

}

pub trait RegistryInternal<E: RegistryEntry>: StartupWriteInternal {

}

pub trait RegistryEntry {

}