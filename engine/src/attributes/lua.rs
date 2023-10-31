use mlua::{FromLua, IntoLua};
use crate::scripting::bridge::Bridge;
use super::value::Attribute;

impl FromLua<'_> for Attribute {
    fn from_lua(value: mlua::Value<'_>, _lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Nil => Ok(Self::Tag),
            mlua::Value::Boolean(i) => Ok(Self::Bool(i)),
            mlua::Value::LightUserData(_) => todo!(),
            mlua::Value::Integer(i) => Ok(Self::Int(i)),
            mlua::Value::Number(i) => Ok(Self::Float(i)),
            mlua::Value::Vector(i) => Ok(Self::Vector(Bridge::from(Bridge(i)))),
            mlua::Value::String(i) => Ok(Self::String(i.to_string_lossy().into())),
            mlua::Value::Table(_i) => {
                // Handle cases like Attribute::Color
                todo!()
            },
            mlua::Value::Function(_) => todo!(),
            mlua::Value::Thread(_) => todo!(),
            mlua::Value::UserData(_) => todo!(),
            mlua::Value::Error(_) => todo!(),
        }
    }
}

impl IntoLua<'_> for Attribute {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        todo!()
    }
}