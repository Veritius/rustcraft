use mlua::FromLua;
use super::id::Identifier;

pub struct ContentPackage {
    pub identifier: Identifier,
    pub owners: Vec<String>,
}

impl FromLua<'_> for ContentPackage {
    fn from_lua(value: mlua::Value<'_>, lua: &'_ mlua::Lua) -> mlua::Result<Self> {
        todo!()
    }
}