mod lua;
mod macroquad_bindings;
mod utils;
use crate::utils::MoreLua;
use macroquad::prelude::*;
use mlua::prelude::*;

macro_rules! call_lua_fn {
    ($lua:expr, $table:ident . $name:ident) => {
        $lua.globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("loaded")?
            .get::<_, LuaTable>(stringify!($table))?
            .get::<_, LuaFunction>(stringify!($name))?
            .call::<_, ()>(())?;
    };
}

#[macroquad::main("test")]
async fn main() -> LuaResult<()> {
    let mut lua = lua::Lua::new()?;
    lua.lua
        .load_module("macroquad", macroquad_bindings::module)?;
    #[cfg(debug_assertions)]
    lua.load_files();
    lua.load_modules()?;
    call_lua_fn!(lua.lua, main.start);
    loop {
        lua.poll();
        call_lua_fn!(lua.lua, main.update);
        next_frame().await;
    }
}
