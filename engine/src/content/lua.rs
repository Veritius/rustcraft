use mlua::{FromLua, IntoLua, Value::Nil};
use super::id::{Identifier, ContentIdentifier};

static KEY_NAMESPACE: &'static str = "namespace";
static KEY_IDENTIFIER: &'static str = "identifier";
static KEY_VARIANT: &'static str = "variant";

impl FromLua<'_> for Identifier {
    fn from_lua(value: mlua::Value<'_>, _lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Nil => Err(mlua::Error::FromLuaConversionError {
                from: "Nil",
                to: "Identifier",
                message: None,
            }),
            mlua::Value::Boolean(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Boolean",
                to: "Identifier",
                message: None,
            }),
            mlua::Value::LightUserData(_) => Err(mlua::Error::FromLuaConversionError {
                from: "LightUserData",
                to: "Identifier",
                message: None,
            }),
            mlua::Value::Integer(i) => {
                Ok(Self::Integer(i))
            },
            mlua::Value::Number(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Number",
                to: "Identifier",
                message: None,
            }),
            mlua::Value::Vector(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Vector",
                to: "Identifier",
                message: None,
            }),
            mlua::Value::String(i) => {
                Ok(Self::BoxedStr(i.to_string_lossy().into()))
            },
            mlua::Value::Table(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Table",
                to: "Identifier",
                message: None,
            }),
            mlua::Value::Function(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Function",
                to: "Identifier",
                message: None,
            }),
            mlua::Value::Thread(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Thread",
                to: "Identifier",
                message: None,
            }),
            mlua::Value::UserData(_) => Err(mlua::Error::FromLuaConversionError {
                from: "UserData",
                to: "Identifier",
                message: None,
            }),
            mlua::Value::Error(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Nil",
                to: "Identifier",
                message: None,
            }),
        }
    }
}

impl IntoLua<'_> for Identifier {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        match self {
            Identifier::StaticStr(i) => i.into_lua(lua),
            Identifier::BoxedStr(i) => i.into_lua(lua),
            Identifier::Integer(i) => i.into_lua(lua),
        }
    }
}

impl FromLua<'_> for ContentIdentifier {
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
            mlua::Value::Integer(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Integer",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::Number(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Number",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::Vector(_) => Err(mlua::Error::FromLuaConversionError {
                from: "Vector",
                to: "ContentIdentifier",
                message: None,
            }),
            mlua::Value::String(_) => {
                todo!()
            },
            mlua::Value::Table(i) => {
                Ok(Self {
                    namespace: i.get(KEY_NAMESPACE)?,
                    identifier: i.get(KEY_IDENTIFIER)?,
                    variant: match i.get(KEY_VARIANT) {
                        Ok(v) => v,
                        Err(_) => None,
                    },
                })
            },
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

impl IntoLua<'_> for ContentIdentifier {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        let table = lua.create_table()?;
        table.set(KEY_NAMESPACE, self.namespace.into_lua(lua)?)?;
        table.set(KEY_IDENTIFIER, self.identifier.into_lua(lua)?)?;
        if self.variant.is_none() {
            table.set(KEY_VARIANT, Nil)?;
        } else {
            table.set(KEY_VARIANT, self.variant.into_lua(lua)?)?;
        }
        Ok(mlua::Value::Table(table))
    }
}