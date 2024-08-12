use super::*;

wrap_functions_for_lua! {
    #[allow(clippy::too_many_arguments)]
    pub wrap mq::shapes::draw_arc as draw_arc
        (x: f32, y: f32, sides: u8, radius: f32, rotation: f32, thickness: f32, arc: f32, color: Color) -> ()

    pub wrap macroquad::shapes::draw_circle as draw_circle
        (x: f32, y: f32, r: f32, color: Color) -> ()

    pub wrap mq::shapes::draw_circle_lines as draw_circle_lines
        (x: f32, y: f32, r: f32, thickness: f32, color: Color) -> ()

    pub wrap mq::shapes::draw_ellipse as draw_ellipse
        (x: f32, y: f32, w: f32, h: f32, rotation: f32, color: Color) -> ()

    pub wrap mq::shapes::draw_ellipse_lines as draw_ellipse_lines
        (x: f32, y: f32, w: f32, h: f32, rotation: f32, thickness: f32, color: Color) -> ()

    pub wrap mq::shapes::draw_hexagon as draw_hexagon
        (x: f32, y: f32, size: f32, border: f32, virtical: bool, border_color: Color, fill_color: Color) -> ()

    pub wrap mq::shapes::draw_line as draw_line
        (x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) -> ()

    pub wrap mq::shapes::draw_poly as draw_poly
        (x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color) -> ()

    pub wrap mq::shapes::draw_poly_lines as draw_poly_lines
        (x: f32, y: f32, sides: u8, radius: f32, rotation: f32, thickness: f32, color: Color) -> ()

    pub wrap mq::shapes::draw_rectangle as draw_rectangle
        (x: f32, y: f32, w: f32, h: f32, color: Color) -> ()

    pub wrap mq::shapes::draw_rectangle_ex as draw_rectangle_ex
        (x: f32, y: f32, w: f32, h: f32, params: DrawRectangleParams) -> ()

    pub wrap mq::shapes::draw_rectangle_lines as draw_rectangle_lines
        (x: f32, y: f32, w: f32, h: f32, thickness: f32, color: Color) -> ()

    pub wrap mq::shapes::draw_rectangle_lines_ex as draw_rectangle_lines_ex
        (x: f32, y: f32, w: f32, h: f32, thickness: f32, params: DrawRectangleParams) -> ()

    pub wrap mq::shapes::draw_triangle as draw_triangle
        (v1: Vec2, v2: Vec2, v3: Vec2, color: Color) -> ()

    pub wrap mq::shapes::draw_triangle_lines as draw_triangle_lines
        (v1: Vec2, v2: Vec2, v3: Vec2, thickness: f32, color: Color) -> ()
}
