use semver::{Version, VersionReq};
use super::id::Identifier;

/// A content package.
/// 
/// Equality (`PartialEq`/`Eq`) only compares the `identifier` and `version` fields.
#[derive(Debug)]
pub struct ContentPackage {
    pub identifier: Identifier,
    pub version: Version,    
    pub dependencies: Vec<(Identifier, VersionReq)>,
    pub owners: Vec<String>,
}

impl PartialEq for ContentPackage {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.version == other.version
    }
}

impl Eq for ContentPackage {}