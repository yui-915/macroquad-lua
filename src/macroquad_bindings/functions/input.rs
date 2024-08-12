use super::*;

wrap_functions_for_lua! {
    pub wrap mq::input::clear_input_queue as clear_input_queue
        () -> ()

    pub wrap mq::input::get_char_pressed as get_char_pressed
        () -> Option<String, char>

    pub wrap mq::input::get_keys_down as get_keys_down
        () -> HashSet<KeyCode, mq::input::KeyCode>

    pub wrap mq::input::get_keys_pressed as get_keys_pressed
        () -> HashSet<KeyCode, mq::input::KeyCode>

    pub wrap mq::input::get_keys_released as get_keys_released
        () -> HashSet<KeyCode, mq::input::KeyCode>

    pub wrap mq::input::get_last_key_pressed as get_last_key_pressed
        () -> Option<KeyCode, mq::input::KeyCode>

    pub wrap mq::input::is_key_down as is_key_down
        (key: KeyCode) -> bool

    pub wrap mq::input::is_key_pressed as is_key_pressed
        (key: KeyCode) -> bool

    pub wrap mq::input::is_key_released as is_key_released
        (key: KeyCode) -> bool

    pub wrap mq::input::is_mouse_button_down as is_mouse_button_down
        (button: MouseButton) -> bool

    pub wrap mq::input::is_mouse_button_pressed as is_mouse_button_pressed
        (button: MouseButton) -> bool

    pub wrap mq::input::is_mouse_button_released as is_mouse_button_released
        (button: MouseButton) -> bool

    pub wrap mq::input::is_quit_requested as is_quit_requested
        () -> bool

    pub wrap mq::input::is_simulating_mouse_with_touch as is_simulating_mouse_with_touch
        () -> bool

    pub wrap mq::input::mouse_delta_position as mouse_delta_position
        () -> Vec2

    pub wrap mq::input::mouse_position as mouse_position
        () -> (f32, f32)

    pub wrap mq::input::mouse_position_local as mouse_position_local
        () -> Vec2

    pub wrap mq::input::mouse_wheel as mouse_wheel
        () -> (f32, f32)

    pub wrap mq::input::prevent_quit as prevent_quit
        () -> ()

    pub wrap mq::input::set_cursor_grab as set_cursor_grab
        (grab: bool) -> ()

    pub wrap mq::input::show_mouse as show_mouse
        (shown: bool) -> ()

    pub wrap mq::input::simulate_mouse_with_touch as simulate_mouse_with_touch
        (option: bool) -> ()

}
