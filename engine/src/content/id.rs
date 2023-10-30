use mlua::{FromLua, IntoLua, Integer};

/// The engine's reserved content package name.
pub(crate) const ENGINE_ID: Identifier = Identifier::StaticStr("engine");

/// An identifier object for a content package or a piece of content.
/// 
/// Implements `PartialEq` and `Eq`, with special behavior.
/// `StaticStr` and `BoxedStr` are equal to themselves and eachother, but `Integer` is only equal to itself.
#[derive(Debug, Clone, Hash)]
pub enum Identifier {
    StaticStr(&'static str),
    BoxedStr(Box<str>),
    Integer(i64),
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StaticStr(l0), Self::StaticStr(r0)) => l0 == r0,
            (Self::StaticStr(l0), Self::BoxedStr(r0)) => l0.as_bytes() == r0.as_bytes(),
            (Self::BoxedStr(l0), Self::BoxedStr(r0)) => l0 == r0,
            (Self::BoxedStr(l0), Self::StaticStr(r0)) => l0.as_bytes() == r0.as_bytes(),
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Identifier {}

impl FromLua<'_> for Identifier {
    fn from_lua(value: mlua::Value<'_>, _lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Nil => Err(mlua::Error::FromLuaConversionError {
                from: "Nil",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::Boolean(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Boolean",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::LightUserData(_) => Err(mlua::Error::FromLuaConversionError {
                from: "LightUserData",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::Integer(int) => Ok(Self::Integer(int)),
            mlua::Value::Number(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Number",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::String(string) => {
                if let Ok(str) = string.to_str() {
                    Ok(Self::BoxedStr(str.into()))
                } else {
                    Err(mlua::Error::FromLuaConversionError {
                        from: "String",
                        to: "ContentIdentifier",
                        message: Some("Failed to convert to valid UTF-8 Rust string".to_string()),
                    })
                }
            },
            mlua::Value::Table(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Table",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::Function(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Function",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::Thread(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Thread",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::UserData(_) => Err(mlua::Error::FromLuaConversionError {
                from: "UserData",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::Error(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Error",
                to: "ContentIdentifier",
                message: None,
            }),
        }
    }
}

impl IntoLua<'_> for Identifier {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        match self {
            Identifier::StaticStr(str) => {
                Ok(mlua::Value::String(lua.create_string(str.as_bytes())?))
            },
            Identifier::BoxedStr(str) => {
                Ok(mlua::Value::String(lua.create_string(str.as_bytes())?))
            },
            Identifier::Integer(int) => {
                Ok(mlua::Value::Integer(Integer::from(int)))
            },
        }
    }
}