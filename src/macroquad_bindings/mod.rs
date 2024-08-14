#[macro_use]
mod macros;

mod types;
use types::*;

mod functions;
use functions::*;

mod modules;

use mlua::prelude::*;

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let color = modules::color::get_table(lua)?;
    let input = modules::input::get_table(lua)?;
    let shapes = modules::shapes::get_table(lua)?;
    let text = modules::text::get_table(lua)?;
    let window = modules::window::get_table(lua)?;

    let prelude = lua_table! {
        using lua;
        extends shapes, input, window, text, color.get::<_, LuaTable>("colors")?;

        fields {
            Color: color.get::<_, Color>("Color")?
        }
    };

    Ok(lua_table! {
        using lua;

        fields {
            shapes: shapes,
            color: color,
            prelude: prelude
        }
    })
}
