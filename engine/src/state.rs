use bevy::prelude::*;

/// The current state of Rustcraft.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, States)]
pub enum EngineState {
    /// Setting up the App and Lua VM, and resolving content package dependencies.
    #[default]
    Initialising,
    /// Letting the Lua VM register definitions and figure everything out before the game starts.
    Registration,
    /// Doing nothing in particular (main menu, etc).
    Idle,
    /// Running the simulation.
    Playing,
}