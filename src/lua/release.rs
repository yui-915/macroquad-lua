use super::{LuaResult, LuaWrapper};

use macroquad_lua_macros::embed_lua_files;

impl LuaWrapper {
    pub fn new() -> LuaResult<Self> {
        Ok(Self {
            lua: mlua::Lua::new(),
            loaded_files: embed_lua_files!("game/src"),
        })
    }
}
