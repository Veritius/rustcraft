//! Mod crate for rustcraft.
//! 
//! Usage example:
//! ```rs
//! #[no_mangle]
//! pub fn metadata() -> ModPackageData {
//!     ModPackageData {
//!         unique_id: "kevin:my_awesome_mod",
//!         name: "Kevin's awesome mod",
//!         description: "An awesome mod that adds awesome things.",
//!         authors: vec!["Kevin"],
//!         version: Version::new(1, 2, 3),
//!         incompatibilities: vec![],
//!         requirements: vec![],
//!     }
//! }
//! 
//! #[no_mangle]
//! pub fn entry_point(&mut App) {
//!     app.insert_resource(MyResource);
//! }
//! ```

pub use rustcraft_engine as engine;
pub use semver;
use semver::{Version, VersionReq};

pub struct ModPackageData {
    /// A unique identifier for the mod. `author:mod` is a good pattern, but not enforced.
    pub unique_id: &'static str,
    /// A display name shown to the user.
    pub name: &'static str,
    /// A brief description of the mod's functionality.
    pub description: &'static str,
    /// The authors of the mod.
    pub authors: Vec<&'static str>,
    /// The mod version.
    pub version: Version,
    /// The engine version this should use.
    pub engine_version: Version,
    /// Any mods that cannot run at the same time as this one.
    pub incompatibilities: Vec<CompatibilityValue>,
    /// Any mods needed to work.
    pub requirements: Vec<CompatibilityValue>,
}

pub struct CompatibilityValue {
    pub unique_id: &'static str,
    pub semver_versionreq: VersionReq,
}