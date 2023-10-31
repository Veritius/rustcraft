//! Dealing with luau's Vector type.

use bevy::prelude::*;
use mlua::Vector;
use super::Bridge;

impl From<Bridge<Vector>> for Bridge<Vec3> {
    fn from(value: Bridge<Vector>) -> Self {
        Self(Vec3 { x: value.x(), y: value.y(), z: value.z() })
    }
}

impl From<Bridge<Vec3>> for Bridge<Vector> {
    fn from(value: Bridge<Vec3>) -> Self {
        Self(Vector::new(value.x, value.y, value.z))
    }
}