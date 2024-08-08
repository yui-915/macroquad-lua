use crate::{
    add_lua_fn, add_lua_fns, impl_from_lua, impl_new, impl_userdata_feilds,
    impl_userdata_feilds_complex, lua_wrap_constructor_fns, make_lua_constants_table,
    make_lua_fns_table, make_lua_fns_table_smol, make_lua_table, reconst, wrap_fn_lua, wrap_type,
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
        // TODO: ALOT OF SHIT
        // TODO: meta_methods
    }
}

wrap_type!(mq::math::Vec2, Vec2);
impl_from_lua!(mq::math::Vec2, Vec2, x y);
// TODO: reconst
impl UserData for Vec2 {
    impl_userdata_feilds!(x y);
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        lua_wrap_constructor_fns!(methods, mq::math::Vec2, Vec2, [
            new x y
        ]);
        // TODO: more constructors
        // TODO: ALOT OF SHIT
        // TODO: meta_methods
    }
}

impl_new!(
    mq::shapes::DrawRectangleParams, DrawRectangleParamsNew,
    offset Vec2, rotation f32, color Color
);
wrap_type!(mq::shapes::DrawRectangleParams, DrawRectangleParams);
impl_from_lua!(mq::shapes::DrawRectangleParams, DrawRectangleParams, offset rotation color);
// no reconst ?
impl UserData for DrawRectangleParams {
    impl_userdata_feilds_complex!(offset Vec2, rotation f32, color Color);
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        lua_wrap_constructor_fns!(methods, mq::shapes::DrawRectangleParams, DrawRectangleParams, [
            new offset rotation color
        ]);
        // TODO: not alot of shit
        // TODO: meta_methods
    }
}

wrap_fn_lua!(mq::shapes::draw_arc, draw_arc, (),
    x f32, y f32,
    sides u8, radius f32,
    rotation f32, thickness f32,
    arc f32, color Color
);
wrap_fn_lua!(mq::shapes::draw_circle, draw_circle, (),
    x f32, y f32,
    r f32, color Color
);
wrap_fn_lua!(mq::shapes::draw_circle_lines, draw_circle_lines, (),
    x f32, y f32,
    r f32, thickness f32,
    color Color
);
wrap_fn_lua!(mq::shapes::draw_ellipse, draw_ellipse, (),
    x f32, y f32,
    w f32, h f32,
    rotation f32, color Color
);
wrap_fn_lua!(mq::shapes::draw_ellipse_lines, draw_ellipse_lines, (),
    x f32, y f32,
    w f32, h f32,
    rotation f32, thickness f32,
    color Color
);
wrap_fn_lua!(mq::shapes::draw_hexagon, draw_hexagon, (),
    x f32, y f32,
    size f32, border f32,
    virtical bool, border_color Color,
    fill_color Color
);
wrap_fn_lua!(mq::shapes::draw_line, draw_line, (),
    x1 f32, y1 f32,
    x2 f32, y2 f32,
    thickness f32, color Color
);
wrap_fn_lua!(mq::shapes::draw_poly, draw_poly, (),
    x f32, y f32,
    sides u8, radius f32,
    rotation f32, color Color
);
wrap_fn_lua!(mq::shapes::draw_poly_lines, draw_poly_lines, (),
    x f32, y f32,
    sides u8, radius f32,
    rotation f32, thickness f32,
    color Color
);
wrap_fn_lua!(mq::shapes::draw_rectangle, draw_rectangle, (),
    x f32, y f32,
    w f32, h f32,
    color Color
);
wrap_fn_lua!(mq::shapes::draw_rectangle_ex, draw_rectangle_ex, (),
    x f32, y f32,
    w f32, h f32,
    params DrawRectangleParams
);
wrap_fn_lua!(mq::shapes::draw_rectangle_lines, draw_rectangle_lines, (),
    x f32, y f32,
    w f32, h f32,
    thickness f32, color Color
);
wrap_fn_lua!(mq::shapes::draw_rectangle_lines_ex, draw_rectangle_lines_ex, (),
    x f32, y f32,
    w f32, h f32,
    thickness f32, params DrawRectangleParams
);
wrap_fn_lua!(mq::shapes::draw_triangle, draw_triangle, (),
    v1 Vec2, v2 Vec2,
    v3 Vec2, color Color
);
wrap_fn_lua!(mq::shapes::draw_triangle_lines, draw_triangle_lines, (),
    v1 Vec2, v2 Vec2,
    v3 Vec2, thickness f32,
    color Color
);

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let prelude = lua.create_table()?;
    let colors = make_lua_constants_table!(lua,
        BEIGE BLACK BLANK BLUE BROWN
        DARKBLUE DARKBROWN DARKGRAY DARKGREEN DARKPURPLE
        GOLD GRAY GREEN LIGHTGRAY LIME
        MAGENTA MAROON ORANGE PINK PURPLE
        RED SKYBLUE VIOLET WHITE YELLOW
    );
    let color = make_lua_table!(lua, [("Color", Color::default())]);

    extend_lua_table!(color, colors);
    extend_lua_table!(prelude, colors);
    color.set("colors", colors)?;
    prelude.set("Color", Color::default())?;

    let shapes = make_lua_fns_table_smol!(lua,
        draw_arc x y sides radius rotation thickness arc color,
        draw_circle x y r color,
        draw_circle_lines x y r thickness color,
        draw_ellipse x y w h rotation color,
        draw_ellipse_lines x y w h rotation thickness color,
        draw_hexagon x y size border virtical border_color fill_color,
        draw_line x1 y1 x2 y2 thickness color,
        draw_poly x y sides radius rotation color,
        draw_poly_lines x y sides radius rotation thickness color,
        draw_rectangle x y w h color,
        draw_rectangle_ex x y w h params,
        draw_rectangle_lines x y w h thickness color,
        draw_rectangle_lines_ex x y w h thickness params,
        draw_triangle v1 v2 v3 color,
        draw_triangle_lines v1 v2 v3 thickness color
    );
    shapes.set("DrawRectangleParams", DrawRectangleParams::default())?;
    extend_lua_table!(prelude, shapes);

    let exports = make_lua_table!(
        lua,
        [("shapes", shapes), ("color", color), ("prelude", prelude)]
    );
    Ok(exports)
}
