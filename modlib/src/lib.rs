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
    pub unique_id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub authors: Vec<&'static str>,
    pub version: Version,
    pub incompatibilities: Vec<CompatibilityValue>,
    pub requirements: Vec<CompatibilityValue>,
}

pub struct CompatibilityValue {
    pub unique_id: &'static str,
    pub semver_versionreq: VersionReq,
}