mod lua;

use lua::LuaWrapper;
mod macroquad_bindings;
use macroquad::prelude::*;
use mlua::prelude::*;

macro_rules! call_lua_fn {
    ($lua:expr, $table:ident . $name:ident()) => {
        $lua.globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("loaded")?
            .get::<_, LuaTable>(stringify!($table))?
            .get::<_, LuaFunction>(stringify!($name))?
            .call::<_, ()>(())
    };
    ($lua:expr, $table:ident . $name:ident ($($arg:tt)*)) => {
        $lua.globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("loaded")?
            .get::<_, LuaTable>(stringify!($table))?
            .get::<_, LuaFunction>(stringify!($name))?
            .call::<_, ()>($($arg)*)
    };
}

macro_rules! result_to_option {
    ($result:expr) => {
        match $result {
            Ok(_) => None,
            Err(e) => Some(e),
        }
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
        let mut last_err: Option<LuaError> = result_to_option!(call_lua_fn!(lua.lua, main.start()));
        loop {
            // MAIN LOOP
            loop {
                if last_err.is_some() {
                    break;
                }
                lua.poll();
                last_err = result_to_option!(call_lua_fn!(lua.lua, main.update()));
                next_frame().await;
            }
            // PANIC LOOP
            let err = last_err.as_ref().unwrap();
            loop {
                if lua.poll() {
                    last_err = None;
                    break;
                }
                match call_lua_fn!(lua.lua, main.panic_update(err.to_string())) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
                next_frame().await;
            }
        }
    }

    #[cfg(not(debug_assertions))]
    {
        let lua = LuaWrapper::new()?;
        lua.load_module("macroquad", macroquad_bindings::module)?;
        lua.load_modules()?;
        let mut last_err: Option<LuaError> = result_to_option!(call_lua_fn!(lua.lua, main.start()));
        // MAIN LOOP
        loop {
            if last_err.is_some() {
                break;
            }
            last_err = result_to_option!(call_lua_fn!(lua.lua, main.update()));
            next_frame().await;
        }
        // PANIC LOOP
        let err = last_err.as_ref().unwrap();
        loop {
            match call_lua_fn!(lua.lua, main.panic_update(err.to_string())) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
            next_frame().await;
        }
    }
}
