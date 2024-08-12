mod lua;

use lua::LuaWrapper;
mod macroquad_bindings;
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
    #[cfg(debug_assertions)]
    {
        let mut lua = LuaWrapper::new()?;
        lua.load_module("macroquad", macroquad_bindings::module)?;
        lua.load_files();
        lua.load_modules()?;
        call_lua_fn!(lua.lua, main.start);
        loop {
            lua.poll();
            call_lua_fn!(lua.lua, main.update);
            next_frame().await;
        }
    }

    #[cfg(not(debug_assertions))]
    {
        let lua = LuaWrapper::new()?;
        lua.load_module("macroquad", macroquad_bindings::module)?;
        lua.load_modules()?;
        call_lua_fn!(lua.lua, main.start);
        loop {
            call_lua_fn!(lua.lua, main.update);
            next_frame().await;
        }
    }
}
