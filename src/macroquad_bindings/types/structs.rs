use macroquad as mq;
use macroquad::color as ColorConstants;

wrap_structs_for_lua! {
    #[derive(Default, Clone, PartialEq)]
    pub wrap mq::color::Color as Color {
        fields { r: f32, g: f32, b: f32, a: f32 }
        impl {
            pub const fn new(color: mq::color::Color) -> Self {
                Self(color)
            }
        }
        UserData {
            auto_impl { clone, clone_from, eq, }
            constructors {
                new (r: f32, g: f32, b: f32, a: f32),
                from_rgba (r: u8, g: u8, b: u8, a: u8),
                from_hex (hex: u32),
                default (),
                from_vec (vec4: Vec4)
            }
            converters {
                to_vec Vec4
            }
        }
        pub constants from ColorConstants {
            BEIGE BLACK BLANK BLUE BROWN
            DARKBLUE DARKBROWN DARKGRAY DARKGREEN DARKPURPLE
            GOLD GRAY GREEN LIGHTGRAY LIME
            MAGENTA MAROON ORANGE PINK PURPLE
            RED SKYBLUE VIOLET WHITE YELLOW
        }
    }

    #[derive(Default)]
    pub wrap mq::math::Vec2 as Vec2 {
        fields { x: f32, y: f32 }
        impl {
            pub const fn new(vec: mq::math::Vec2) -> Self {
                Self(vec)
            }
        }
        UserData {
            constructors {
                new (x: f32, y: f32)
            }
        }
    }

    #[derive(Default)]
    pub wrap mq::math::Vec4 as Vec4 {
        fields { x: f32, y: f32, z: f32, w: f32 }
        impl {
            pub const fn new(vec: mq::math::Vec4) -> Self {
                Self(vec)
            }
        }
        UserData {
            constructors {
                new (x: f32, y: f32, z: f32, w: f32)
            }
        }
    }

    #[derive(Default, Clone)]
    pub wrap mq::shapes::DrawRectangleParams as DrawRectangleParams {
        fields { offset: Vec2, rotation: f32, color: Color }
        impl {
            pub const fn new(params: mq::shapes::DrawRectangleParams) -> Self {
                Self(params)
            }
        }
        UserData {
            auto_impl { clone, clone_from, }
            constructors {
                default ()
            }
        }
    }

    #[derive(Default, Clone)]
    pub wrap mq::text::TextDimensions as TextDimensions {
        fields { width: f32, height: f32, offset_y: f32 }
        UserData {
            auto_impl { clone, clone_from, }
            constructors {
                default ()
            }
        }
    }
}
