use super::*;

wrap_functions_for_lua! {
    pub wrap mq::time::get_fps as get_fps () -> i32
    pub wrap mq::time::get_frame_time as get_frame_time () -> f32
    pub wrap mq::time::get_time as get_time () -> f64
}
