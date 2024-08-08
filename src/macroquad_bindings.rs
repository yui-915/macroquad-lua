use crate::{
    add_lua_fn, add_lua_fns, impl_from_lua, impl_userdata_feilds, lua_wrap_constructor_fns,
    make_lua_constants_table, make_lua_fns_table, make_lua_table, reconst, wrap_fn_lua, wrap_type,
};
use crate::{extend_lua_table, lua_wrap_constructor_fn};
use macroquad as mq;
use mlua::prelude::*;
use mlua::UserData;

wrap_type!(mq::color::Color, Color);
impl_from_lua!(mq::color::Color, Color, r g b a);
reconst!(Color, mq::color,
    BEIGE BLACK BLANK BLUE BROWN
    DARKBLUE DARKBROWN DARKGRAY DARKGREEN DARKPURPLE
    GOLD GRAY GREEN LIGHTGRAY LIME
    MAGENTA MAROON ORANGE PINK PURPLE
    RED SKYBLUE VIOLET WHITE YELLOW
);
impl UserData for Color {
    impl_userdata_feilds!(r g b a);
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        lua_wrap_constructor_fns!(methods, mq::color::Color, Color, [
            new r g b a,
            from_rgba r g b a,
            from_hex hex,
            default
        ]);
        // TODO: to_vec from_vec fmt default from
        // TODO: meta_methods
    }
}

wrap_fn_lua!(mq::shapes::draw_rectangle, draw_rectangle, (), x f32, y f32, w f32, h f32, c Color);

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let prelude = lua.create_table()?;
    let colors = make_lua_constants_table!(lua,
        BEIGE BLACK BLANK BLUE BROWN
        DARKBLUE DARKBROWN DARKGRAY DARKGREEN DARKPURPLE
        GOLD GRAY GREEN LIGHTGRAY LIME
        MAGENTA MAROON ORANGE PINK PURPLE
        RED SKYBLUE VIOLET WHITE YELLOW
    );
    let color = make_lua_table!(lua, [("Color", Color(mq::color::BLANK))]);

    extend_lua_table!(color, colors);
    extend_lua_table!(prelude, colors);
    color.set("colors", colors)?;
    prelude.set("Color", Color(mq::color::BLANK))?;

    let shapes = make_lua_fns_table!(lua, [
        ("draw_rectangle", draw_rectangle, x y w h c)
    ]);
    extend_lua_table!(prelude, shapes);

    let exports = make_lua_table!(
        lua,
        [("shapes", shapes), ("color", color), ("prelude", prelude)]
    );
    Ok(exports)
}
