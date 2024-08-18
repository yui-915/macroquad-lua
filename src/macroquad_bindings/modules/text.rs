use super::*;

pub fn get_table(lua: &Lua) -> LuaResult<LuaTable> {
    Ok(lua_table! {
        using lua;

        functions {
            draw_text (text x y font_size color),
            draw_multiline_text (text x y font_size line_distance_factor color),
            camera_font_scale (world_font_size)
        }
    })
}
