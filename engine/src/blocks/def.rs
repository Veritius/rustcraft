//! Static blocks that are always added to the game.

use crate::{content::id::{ENGINE_ID, ContentIdentifier, IdentifierSegment}, attributes::value::Attribute};

/// An absence of a voxel.
pub fn void() -> (ContentIdentifier, Vec<(ContentIdentifier, Attribute)>) {
    (
        ContentIdentifier {
            namespace: ENGINE_ID,
            identifier: IdentifierSegment::from("air"),
            variant: None,
        },
        vec![]
    )
}