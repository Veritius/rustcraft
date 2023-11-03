use bevy::prelude::*;

/// The current state of Rustcraft.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, States)]
pub enum EngineState {
    /// Setting up the App and resolving content package dependencies.
    #[default]
    Initialising,
    /// Defining prototypes and figuring out definitions.
    Registration,
    /// Doing nothing in particular (main menu, etc).
    Idle,
    /// Running the simulation.
    Playing,
}