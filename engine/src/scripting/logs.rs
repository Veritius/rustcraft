use bevy::log::*;
use mlua::UserData;

/// A way to use the `logging` crate from Lua code. Intended for use as a global.
pub struct LuaLogger;

impl UserData for LuaLogger {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("error", |_lua, (origin, message): (String, String)| {
            error!("Error log in Lua: [{origin}] {message}");
            Ok(())
        });

        methods.add_function("warn", |_lua, (origin, message): (String, String)| {
            warn!("Warning log in Lua: [{origin}] {message}");
            Ok(())
        });

        methods.add_function("info", |_lua, (origin, message): (String, String)| {
            info!("Info log in Lua: [{origin}] {message}");
            Ok(())
        });

        methods.add_function("debug", |_lua, (origin, message): (String, String)| {
            debug!("Debug log in Lua: [{origin}] {message}");
            Ok(())
        });

        methods.add_function("trace", |_lua, (origin, message): (String, String)| {
            trace!("Trace log in Lua: [{origin}] {message}");
            Ok(())
        });
    }
}