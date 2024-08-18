use super::*;

pub fn get_table(lua: &Lua) -> LuaResult<LuaTable> {
    let colors = lua_table! {
        using lua;

        constants {
            BEIGE BLACK BLANK BLUE BROWN
            DARKBLUE DARKBROWN DARKGRAY DARKGREEN DARKPURPLE
            GOLD GRAY GREEN LIGHTGRAY LIME
            MAGENTA MAROON ORANGE PINK PURPLE
            RED SKYBLUE VIOLET WHITE YELLOW
        }
    };

    Ok(lua_table! {
        using lua;
        extends colors;

        functions {
            hsl_to_rgb (h s l),
            rgb_to_hsl (color)
        }

        fields {
            Color: Color::new(Default::default()),
            colors: colors
        }
    })
}
