use mlua::{FromLua, IntoLua};
use crate::scripting::bridge::Bridge;
use super::value::Attribute;

impl FromLua<'_> for Attribute {
    fn from_lua(value: mlua::Value<'_>, _lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Nil => Ok(Self::Tag),
            mlua::Value::Boolean(i) => Ok(Self::Bool(i)),
            mlua::Value::Integer(i) => Ok(Self::Int(i)),
            mlua::Value::Number(i) => Ok(Self::Float(i)),
            mlua::Value::Vector(i) => Ok(Self::Vector(Bridge::from(Bridge(i)))),
            mlua::Value::String(i) => Ok(Self::String(i.to_string_lossy().into())),
            mlua::Value::Table(_i) => {
                // Handle cases like Attribute::Color
                todo!()
            },
            _ => {
                Err(mlua::Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "attribute",
                    message: None,
                })
            }
        }
    }
}

impl IntoLua<'_> for Attribute {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        match self {
            Attribute::Tag => Ok(mlua::Value::Boolean(true)),
            Attribute::Bool(i) => Ok(mlua::Value::Boolean(i)),
            Attribute::Int(i) => Ok(mlua::Value::Integer(i)),
            Attribute::Float(i) => Ok(mlua::Value::Number(i)),
            Attribute::String(i) => Ok(i.into_lua(lua)?),
            Attribute::Color(_) => todo!(),
            Attribute::Vector(i) => Ok(i.into_lua(lua)?),
            Attribute::Dyn(_) => todo!(),
        }
    }
}