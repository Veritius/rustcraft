use std::any::Any;
use crate::content::id::Identifier;

/// An identifier used to access a [BlockAttribute].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeIdentifier {
    pub namespace: Identifier,
    pub identifier: Identifier,
}

/// An attribute that is attached to a block definition.
pub trait BlockAttribute: Send + Sync + Any {}
impl<T: Send + Sync + Any> BlockAttribute for T {}