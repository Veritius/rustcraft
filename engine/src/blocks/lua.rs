use std::sync::Arc;
use mlua::{UserData, IntoLua};
use crate::content::id::ContentIdentifier;
use super::registry::BlockRegistryInner;

/// Block registry object added to the Lua VM as a global.
pub struct LuaBlockRegistry(Arc<BlockRegistryInner>);

impl UserData for LuaBlockRegistry {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("block_exists", |_, this, id: ContentIdentifier| {
            Ok(this.0.block_exists(&id))
        });

        methods.add_method("block_id", |lua, this, id: ContentIdentifier| {
            Ok(match this.0.block_id(&id) {
                Some(v) => v.into_lua(lua).unwrap(),
                None => mlua::Value::Nil,
            })
        });
    }
}