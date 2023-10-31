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
            mlua::Value::String(i) => {
                // Read a ContentIdentifier from the following form:
                // namespace:identifier/variant

                let i = String::from_utf8_lossy(i.as_bytes()).into_owned();
                
                if !i.contains(':') {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "string",
                        to: "contentidentifier",
                        message: Some("String did not contain a colon".to_string()),
                    });
                }

                let mut colon_split = i.split(':');

                // Namespace
                let namespace = colon_split.next().unwrap();
                if namespace.len() == 0 {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "string",
                        to: "contentidentifier",
                        message: Some("Length of namespace segment was zero".to_string()),
                    });
                }
                let namespace = Identifier::from(namespace);

                // Identifier
                let identifier = colon_split.next().unwrap();
                if identifier.len() == 0 {
                    return Err(mlua::Error::FromLuaConversionError {
                        from: "string",
                        to: "contentidentifier",
                        message: Some("Length of identifier segment was zero".to_string()),
                    });
                }

                // Variant
                if identifier.contains('/') {
                    let mut slash_split = i.split('/');
                    let identifier = slash_split.next().unwrap();
                    let variant = slash_split.next().unwrap();

                    if identifier.len() == 0 {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: "string",
                            to: "contentidentifier",
                            message: Some("Length of identifier segment was zero".to_string()),
                        });
                    }

                    return Ok(Self {
                        namespace,
                        identifier: Identifier::from(identifier),
                        variant: Some(Identifier::from(variant))
                    })
                } else {
                    return Ok(Self {
                        namespace,
                        identifier: Identifier::from(identifier),
                        variant: None,
                    })
                }
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