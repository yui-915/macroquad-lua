use super::*;

wrap_functions_for_lua! {
    pub wrap mq::color::hsl_to_rgb as hsl_to_rgb
        (h: f32, s: f32, l: f32) -> Color

    pub wrap mq::color::rgb_to_hsl as rgb_to_hsl
        (color: Color) -> (f32, f32, f32)
}
