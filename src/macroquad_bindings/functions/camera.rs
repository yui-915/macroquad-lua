use super::*;

wrap_functions_for_lua! {
    pub wrap mq::camera::pop_camera_state as pop_camera_state () -> ()
    pub wrap mq::camera::push_camera_state as push_camera_state () -> ()
    pub wrap mq::camera::set_default_camera as set_default_camera () -> ()
}
