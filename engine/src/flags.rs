//! Values solved at compile time to ensure compatibility between peers in multiplayer.

/// The version of the engine crate.
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

/// If the `big_ids` feature flag is enabled.
pub static BIG_IDS: bool = if cfg!(feature="big_ids") { true } else { false };