// TODO: REFACTOR

use std::collections::HashSet;
use std::marker::PhantomData;

use crate::{
    add_lua_fn, add_lua_fns, extend_lua_table, impl_from_lua, impl_new, impl_userdata_feilds,
    impl_userdata_feilds_complex, lua_wrap_constructor_fn, lua_wrap_constructor_fns,
    make_lua_constants_table, make_lua_enum_table, make_lua_fns_table, make_lua_fns_table_smol,
    make_lua_table, reconst, wrap_enum, wrap_fn_lua, wrap_type,
};
use macroquad as mq;
use macroquad::{
    color::Color as MacroquadColor, input::KeyCode as MacroquadKeyCode,
    input::MouseButton as MacroquadMouseButton, math::Vec2 as MacroquadVec2,
    shapes::DrawRectangleParams as MacroquadDrawRectangleParams,
};
use mlua::prelude::*;
use mlua::UserData;

wrap_type!(MacroquadColor, Color);
impl_from_lua!(MacroquadColor, Color, r g b a);
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
        lua_wrap_constructor_fns!(methods, MacroquadColor, Color, [
            new r g b a,
            from_rgba r g b a,
            from_hex hex,
            default
        ]);
        // TODO: ALOT OF SHIT
        // TODO: meta_methods
    }
}

wrap_type!(MacroquadVec2, Vec2);
impl_from_lua!(MacroquadVec2, Vec2, x y);
// TODO: reconst
impl UserData for Vec2 {
    impl_userdata_feilds!(x y);
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        lua_wrap_constructor_fns!(methods, MacroquadVec2, Vec2, [
            new x y
        ]);
        // TODO: more constructors
        // TODO: ALOT OF SHIT
        // TODO: meta_methods
    }
}

impl_new!(
    MacroquadDrawRectangleParams, DrawRectangleParamsNew,
    offset Vec2, rotation f32, color Color
);
wrap_type!(MacroquadDrawRectangleParams, DrawRectangleParams);
impl_from_lua!(MacroquadDrawRectangleParams, DrawRectangleParams, offset rotation color);
// no reconst ?
impl UserData for DrawRectangleParams {
    impl_userdata_feilds_complex!(offset Vec2, rotation f32, color Color);
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        lua_wrap_constructor_fns!(methods, MacroquadDrawRectangleParams, DrawRectangleParams, [
            new offset rotation color
        ]);
        // TODO: not alot of shit
        // TODO: meta_methods
    }
}

wrap_enum!(MacroquadKeyCode, KeyCode,
    Space Apostrophe Comma Minus Period Slash
    Key0 Key1 Key2 Key3 Key4 Key5 Key6 Key7 Key8 Key9
    Semicolon Equal
    A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
    LeftBracket Backslash RightBracket GraveAccent
    World1 World2
    Escape Enter Tab Backspace Insert Delete
    Right Left Down Up
    PageUp PageDown Home End
    CapsLock ScrollLock NumLock PrintScreen Pause
    F1 F2 F3 F4 F5 F6 F7 F8 F9 F10 F11 F12 F13
    F14 F15 F16 F17 F18 F19 F20 F21 F22 F23 F24 F25
    Kp0 Kp1 Kp2 Kp3 Kp4 Kp5 Kp6 Kp7 Kp8 Kp9
    KpDecimal KpDivide KpMultiply KpSubtract KpAdd KpEnter KpEqual
    LeftShift LeftControl LeftAlt LeftSuper
    RightShift RightControl RightAlt RightSuper
    Menu Unknown
);
wrap_enum!(MacroquadMouseButton, MouseButton,
    Left Middle Right Unknown
);

struct WrappedHashSet<T, D>(HashSet<D>, PhantomData<T>);
impl<T, D> From<WrappedHashSet<T, D>> for HashSet<D> {
    fn from(wrapped: WrappedHashSet<T, D>) -> Self {
        wrapped.0
    }
}
impl<T, D> From<HashSet<D>> for WrappedHashSet<T, D> {
    fn from(set: HashSet<D>) -> Self {
        WrappedHashSet(set, PhantomData)
    }
}
impl<'lua, T: From<D> + IntoLua<'lua>, D> IntoLua<'lua> for WrappedHashSet<T, D> {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        for key in self.0 {
            table.set(T::from(key), true)?;
        }
        table.into_lua(lua)
    }
}

struct WrappedOption<T, D>(Option<D>, PhantomData<T>);
impl<T, D> From<WrappedOption<T, D>> for Option<D> {
    fn from(wrapped: WrappedOption<T, D>) -> Self {
        wrapped.0
    }
}
impl<T, D> From<Option<D>> for WrappedOption<T, D> {
    fn from(option: Option<D>) -> Self {
        WrappedOption(option, PhantomData)
    }
}
impl<'lua, T: From<D> + IntoLua<'lua>, D> IntoLua<'lua> for WrappedOption<T, D> {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue> {
        match self.0 {
            Some(value) => T::from(value).into_lua(lua),
            None => Ok(LuaValue::Nil),
        }
    }
}

struct WrappedVec<T, D>(Vec<D>, PhantomData<T>);
impl<T, D> From<WrappedVec<T, D>> for Vec<D> {
    fn from(wrapped: WrappedVec<T, D>) -> Self {
        wrapped.0
    }
}
impl<T, D> From<Vec<D>> for WrappedVec<T, D> {
    fn from(vec: Vec<D>) -> Self {
        WrappedVec(vec, PhantomData)
    }
}
impl<'lua, T: From<D> + IntoLua<'lua>, D> IntoLua<'lua> for WrappedVec<T, D> {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue> {
        let table = lua.create_table()?;
        for value in self.0 {
            table.set(T::from(value), true)?;
        }
        table.into_lua(lua)
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
wrap_fn_lua!(mq::input::is_key_down, is_key_down, bool,
    key KeyCode
);
wrap_fn_lua!(mq::input::is_key_pressed, is_key_pressed, bool,
    key KeyCode
);
wrap_fn_lua!(mq::input::is_key_released, is_key_released, bool,
    key KeyCode
);
wrap_fn_lua!(mq::input::is_mouse_button_down, is_mouse_button_down, bool,
    button MouseButton
);
wrap_fn_lua!(mq::input::is_mouse_button_pressed, is_mouse_button_pressed, bool,
    button MouseButton
);
wrap_fn_lua!(mq::input::is_mouse_button_released, is_mouse_button_released, bool,
    button MouseButton
);
wrap_fn_lua!(mq::input::clear_input_queue, clear_input_queue, (),);
wrap_fn_lua!(mq::input::is_quit_requested, is_quit_requested, bool,);
wrap_fn_lua!(
    mq::input::is_simulating_mouse_with_touch,
    is_simulating_mouse_with_touch,
    bool,
);
wrap_fn_lua!(mq::input::mouse_delta_position, mouse_delta_position, Vec2,);
wrap_fn_lua!(mq::input::mouse_position_local, mouse_position_local, Vec2,);
wrap_fn_lua!(mq::input::prevent_quit, prevent_quit, (),);
wrap_fn_lua!(mq::input::set_cursor_grab, set_cursor_grab, (), grab bool);
wrap_fn_lua!(mq::input::show_mouse, show_mouse, (), shown bool);
wrap_fn_lua!(mq::input::simulate_mouse_with_touch, simulate_mouse_with_touch, (), option bool);

fn get_char_pressed() -> WrappedOption<String, char> {
    mq::input::get_char_pressed().into()
}
fn get_keys_down() -> WrappedHashSet<KeyCode, MacroquadKeyCode> {
    mq::input::get_keys_down().into()
}
fn get_keys_pressed() -> WrappedHashSet<KeyCode, MacroquadKeyCode> {
    mq::input::get_keys_pressed().into()
}
fn get_keys_released() -> WrappedHashSet<KeyCode, MacroquadKeyCode> {
    mq::input::get_keys_released().into()
}
fn get_last_key_pressed() -> WrappedOption<KeyCode, MacroquadKeyCode> {
    mq::input::get_last_key_pressed().into()
}
fn mouse_position() -> (f32, f32) {
    mq::input::mouse_position()
}
fn mouse_wheel() -> (f32, f32) {
    mq::input::mouse_wheel()
}

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

    let input = make_lua_fns_table_smol!(lua,
       clear_input_queue,
        get_char_pressed,
        get_keys_down,
        get_keys_pressed,
        get_keys_released,
        get_last_key_pressed,
        is_key_down key,
        is_key_pressed key,
        is_key_released key,
        is_mouse_button_down button,
        is_mouse_button_pressed button,
        is_mouse_button_released button,
        is_quit_requested,
        is_simulating_mouse_with_touch,
        mouse_delta_position,
        mouse_position,
        mouse_position_local,
        mouse_wheel,
        prevent_quit,
        set_cursor_grab grab,
        show_mouse shown,
        simulate_mouse_with_touch option
    );
    input.set(
        "KeyCode",
        make_lua_enum_table!(lua,
            Space Apostrophe Comma Minus Period Slash
            Key0 Key1 Key2 Key3 Key4 Key5 Key6 Key7 Key8 Key9
            Semicolon Equal
            A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
            LeftBracket Backslash RightBracket GraveAccent
            World1 World2
            Escape Enter Tab Backspace Insert Delete
            Right Left Down Up
            PageUp PageDown Home End
            CapsLock ScrollLock NumLock PrintScreen Pause
            F1 F2 F3 F4 F5 F6 F7 F8 F9 F10 F11 F12 F13
            F14 F15 F16 F17 F18 F19 F20 F21 F22 F23 F24 F25
            Kp0 Kp1 Kp2 Kp3 Kp4 Kp5 Kp6 Kp7 Kp8 Kp9
            KpDecimal KpDivide KpMultiply KpSubtract KpAdd KpEnter KpEqual
            LeftShift LeftControl LeftAlt LeftSuper
            RightShift RightControl RightAlt RightSuper
            Menu Unknown
        ),
    )?;
    input.set(
        "MouseButton",
        make_lua_enum_table!(lua, Left Middle Right Unknown),
    )?;
    extend_lua_table!(prelude, input);

    let exports = make_lua_table!(
        lua,
        [("shapes", shapes), ("color", color), ("prelude", prelude)]
    );
    Ok(exports)
}
