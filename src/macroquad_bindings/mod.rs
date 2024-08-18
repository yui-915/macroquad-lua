#[macro_use]
mod macros;

mod types;
use types::*;

mod functions;
use functions::*;

mod modules;

use mlua::prelude::*;

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let camera = modules::camera::get_table(lua)?;
    let color = modules::color::get_table(lua)?;
    let extra = modules::extra::get_table(lua)?;
    let input = modules::input::get_table(lua)?;
    let shapes = modules::shapes::get_table(lua)?;
    let text = modules::text::get_table(lua)?;
    let time = modules::time::get_table(lua)?;
    let window = modules::window::get_table(lua)?;
    let rand = modules::rand::get_table(lua)?;

    let prelude = lua_table! {
        using lua;
        extends input, shapes, text, time, window, camera, color.get::<_, LuaTable>("colors")?;

        fields {
            Color: color.get::<_, Color>("Color")?,
            rand: &rand
        }
    };

    Ok(lua_table! {
        using lua;

        fields {
            camera: camera,
            color: color,
            extra: extra,
            input: input,
            prelude: prelude,
            rand: rand,
            shapes: shapes,
            text: text,
            time: time,
            window: window
        }
    })
}
