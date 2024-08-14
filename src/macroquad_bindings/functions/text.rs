use super::*;

wrap_functions_for_lua! {
    pub wrap mq::text::draw_text as draw_text
        (text: String:as_str, x: f32, y: f32, font_size: f32, color: Color) -> TextDimensions

    pub wrap mq::text::draw_multiline_text as draw_multiline_text
        (
            text: String:as_str,
            x: f32, y: f32, font_size: f32,
            line_distance_factor: Option<f32, f32>, color: Color
        ) -> ()
}
