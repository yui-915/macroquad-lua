use macroquad as mq;
use mlua::prelude::*;

wrap_enums_for_lua! {
    pub wrap mq::input::MouseButton as MouseButton {
        Left Middle Right Unknown
    }

    pub wrap mq::input::KeyCode as KeyCode {
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
    }
}
