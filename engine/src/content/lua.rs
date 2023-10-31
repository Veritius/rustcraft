use mlua::{FromLua, IntoLua, Value::Nil};
use super::id::{Identifier, ContentIdentifier};

static KEY_NAMESPACE: &'static str = "namespace";
static KEY_IDENTIFIER: &'static str = "identifier";
static KEY_VARIANT: &'static str = "variant";

impl FromLua<'_> for Identifier {
    fn from_lua(value: mlua::Value<'_>, _lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Integer(i) => {
                Ok(Self::Integer(i))
            },
            mlua::Value::String(i) => {
                Ok(Self::BoxedStr(i.to_string_lossy().into()))
            },
            _ => {
                Err(mlua::Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "identifier",
                    message: None,
                })
            }
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
            _ => {
                Err(mlua::Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "contentidentifier",
                    message: None,
                })
            }
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