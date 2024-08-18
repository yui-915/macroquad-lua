use super::*;

pub fn get_table(lua: &Lua) -> LuaResult<LuaTable> {
    Ok(lua_table! {
        using lua;

        functions {
            get_fps (),
            get_frame_time (),
            get_time ()
        }
    })
}
