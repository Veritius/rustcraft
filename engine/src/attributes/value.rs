use std::any::Any;

/// An attribute that can be stored in an [Attributes](super::map::Attributes) map.
#[derive(Debug)]
pub enum Attribute {
    /// No associated information.
    Tag,

    /// A boolean value.
    Bool(bool),

    /// A signed integer value.
    Int(i32),

    /// A floating point value.
    Float(f64),

    /// A string value.
    String(Box<str>),

    /// A dynamically typed attribute.
    Dyn(Box<dyn DynAttribute>),
}

/// A dynamically typed attribute.
pub trait DynAttribute: std::fmt::Debug + Send + Sync + Any {}
impl<T: std::fmt::Debug + Send + Sync + Any> DynAttribute for T {}