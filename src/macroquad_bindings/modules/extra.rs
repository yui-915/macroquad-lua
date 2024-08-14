use super::*;

pub fn get_table(lua: &Lua) -> LuaResult<LuaTable> {
    let global_use = lua.create_function(|lua, table: LuaTable| {
        table.for_each(|k: String, v: LuaValue| lua.globals().set(k, v))
    })?;

    Ok(lua_table! {
        using lua;
        fields {
            global_use: global_use
        }
    })
}
