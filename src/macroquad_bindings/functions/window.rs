use super::*;

wrap_functions_for_lua! {
    pub wrap mq::window::clear_background as clear_background
        (color: Color) -> ()

    pub wrap mq::window::screen_height as screen_height
        () -> f32

    pub wrap mq::window::screen_width as screen_width
        () -> f32

    pub wrap mq::window::set_fullscreen as set_fullscreen
        (fullscreen: bool) -> ()

    pub wrap mq::window::request_new_screen_size as request_new_screen_size
        (width: f32, height: f32) -> ()

    pub wrap mq::window::screen_dpi_scale as screen_dpi_scale
        () -> f32
}
