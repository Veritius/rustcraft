use std::any::Any;
use bevy::prelude::*;
use crate::scripting::bridge::Bridge;

/// An attribute that can be stored in an [Attributes](super::map::Attributes) map.
#[derive(Debug)]
pub enum Attribute {
    /// No associated information. `Nil` in Lua.
    Tag,

    /// A boolean value. `Boolean` in Lua.
    Bool(bool),

    /// A signed integer value. `Integer` in Lua.
    Int(i32),

    /// A floating point value. `Number` in Lua.
    Float(f64),

    /// A string value. `String` in Lua.
    String(Box<str>),

    /// An RGB color value. A table in Lua.
    Color(Bridge<Color>),

    /// A 3 dimensional vector. `Vector` in Lua.
    Vector(Bridge<Vec3>),

    /// A dynamically typed attribute. Not accessible in Lua.
    Dyn(Box<dyn DynAttribute>),
}

/// A dynamically typed attribute.
pub trait DynAttribute: std::fmt::Debug + Send + Sync + Any {}
impl<T: std::fmt::Debug + Send + Sync + Any> DynAttribute for T {}