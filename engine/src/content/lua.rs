use mlua::{FromLua, IntoLua, UserData};
use super::id::{IdentifierSegment, ContentIdentifier};

static KEY_NAMESPACE: &'static str = "namespace";
static KEY_IDENTIFIER: &'static str = "identifier";
static KEY_VARIANT: &'static str = "variant";

impl FromLua<'_> for IdentifierSegment {
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

impl IntoLua<'_> for IdentifierSegment {
    fn into_lua(self, lua: &'_ mlua::Lua) -> mlua::Result<mlua::Value<'_>> {
        match self {
            IdentifierSegment::StaticStr(i) => i.into_lua(lua),
            IdentifierSegment::BoxedStr(i) => i.into_lua(lua),
            IdentifierSegment::Integer(i) => i.into_lua(lua),
        }
    }
}

impl UserData for ContentIdentifier {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("namespace", |_lua, this| {
            Ok(this.namespace.clone())
        });
        fields.add_field_method_set("namespace", |_lua, this, val| {
            this.namespace = val;
            Ok(())
        });

        fields.add_field_method_get("identifier", |_lua, this| {
            Ok(this.identifier.clone())
        });
        fields.add_field_method_set("identifier", |_lua, this, val| {
            this.identifier = val;
            Ok(())
        });

        fields.add_field_method_get("variant", |_lua, this| {
            Ok(this.variant.clone())
        });
        fields.add_field_method_set("variant", |_lua, this, val| {
            this.variant = val;
            Ok(())
        });
    }
}

impl FromLua<'_> for ContentIdentifier {
    fn from_lua(value: mlua::Value<'_>, _lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::String(i) => {
                let v = ContentIdentifier::try_from(i.to_string_lossy().to_string().as_str());
                match v {
                    Ok(i) => { return Ok(i) },
                    Err(_) => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: "string",
                            to: "contentidentifier",
                            message: None,
                        })
                    },
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