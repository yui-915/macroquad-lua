use super::*;

pub fn get_table(lua: &Lua) -> LuaResult<LuaTable> {
    Ok(lua_table! {
        using lua;

        functions {
            pop_camera_state (),
            push_camera_state (),
            set_default_camera ()
        }
    })
}
