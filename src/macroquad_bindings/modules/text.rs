use super::*;

pub fn get_table(lua: &Lua) -> LuaResult<LuaTable> {
    Ok(lua_table! {
        using lua;

        functions {
            draw_text (text x y font_size color)
        }
    })
}
