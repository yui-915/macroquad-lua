use super::*;

pub fn get_table(lua: &Lua) -> LuaResult<LuaTable> {
    Ok(lua_table! {
        using lua;

        functions {
            clear_background (color),
            screen_height (),
            screen_width (),
            set_fullscreen (fullscreen),
            request_new_screen_size (width height),
            screen_dpi_scale ()
        }
    })
}
