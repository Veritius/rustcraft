use semver::{Version, VersionReq};
use serde::Deserialize;
use super::id::IdentifierSegment;

/// A content package.
/// 
/// Equality (`PartialEq`/`Eq`) only compares the `identifier` and `version` fields.
#[derive(Debug, Deserialize)]
pub struct ContentPackage {
    pub identifier: IdentifierSegment,
    pub version: Version,    
    pub dependencies: Vec<(IdentifierSegment, VersionReq)>,
    pub owners: Vec<String>,
}

impl PartialEq for ContentPackage {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.version == other.version
    }
}

impl Eq for ContentPackage {}