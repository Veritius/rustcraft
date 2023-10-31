use bevy::prelude::*;
use mlua::{FromLua, IntoLua, Vector};
use super::Bridge;

impl FromLua<'_> for Bridge<Color> {
    fn from_lua(value: mlua::Value<'_>, lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        todo!()
    }
}

impl IntoLua<'_> for Bridge<Color> {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        todo!()
    }
}

impl FromLua<'_> for Bridge<Vec2> {
    fn from_lua(value: mlua::Value<'_>, lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        todo!()
    }
}

impl IntoLua<'_> for Bridge<Vec2> {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        todo!()
    }
}

impl FromLua<'_> for Bridge<Vec3> {
    fn from_lua(value: mlua::Value<'_>, lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        Ok(Bridge(Vector::from_lua(value, lua)?).into())
    }
}

impl IntoLua<'_> for Bridge<Vec3> {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        Into::<Bridge<Vector>>::into(self).into_lua(lua)
    }
}