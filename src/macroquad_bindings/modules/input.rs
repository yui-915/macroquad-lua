use super::*;

pub fn get_table(lua: &Lua) -> LuaResult<LuaTable> {
    Ok(lua_table! {
        using lua;

        functions {
            clear_input_queue (),
            get_char_pressed (),
            get_keys_down (),
            get_keys_pressed (),
            get_keys_released (),
            get_last_key_pressed (),
            is_key_down (key),
            is_key_pressed (key),
            is_key_released (key),
            is_mouse_button_down (button),
            is_mouse_button_pressed (button),
            is_mouse_button_released (button),
            is_quit_requested (),
            is_simulating_mouse_with_touch (),
            mouse_delta_position (),
            mouse_position (),
            mouse_position_local (),
            mouse_wheel (),
            prevent_quit (),
            set_cursor_grab (grab),
            show_mouse (shown),
            simulate_mouse_with_touch (option)
        }

        enums {
            KeyCode {
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
            },
            MouseButton {
                Left Middle Right Unknown
            }
        }
    })
}
