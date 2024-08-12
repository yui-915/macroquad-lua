use super::*;

pub fn get_table(lua: &Lua) -> LuaResult<LuaTable> {
    Ok(lua_table! {
        using lua;

        functions {
            draw_arc (x y sides radius rotation thickness arc color),
            draw_circle (x y r color),
            draw_circle_lines (x y r thickness color),
            draw_ellipse (x y w h rotation color),
            draw_ellipse_lines (x y w h rotation thickness color),
            draw_hexagon (x y size border virtical border_color fill_color),
            draw_line (x1 y1 x2 y2 thickness color),
            draw_poly (x y sides radius rotation color),
            draw_poly_lines (x y sides radius rotation thickness color),
            draw_rectangle (x y w h color),
            draw_rectangle_ex (x y w h params),
            draw_rectangle_lines (x y w h thickness color),
            draw_rectangle_lines_ex (x y w h thickness params),
            draw_triangle (v1 v2 v3 color),
            draw_triangle_lines (v1 v2 v3 thickness color)
        }

        fields {
            DrawRectangleParams: DrawRectangleParams::default()
        }
    })
}
