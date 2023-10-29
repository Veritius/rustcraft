use std::any::TypeId;

/// An identifier object for a content package or a piece of content.
pub enum ContentIdentifier {
    String(String),
    Integer(u64),
}