use macroquad as mq;
use macroquad::color as ColorConstants;

wrap_structs_for_lua! {
    #[derive(Default)]
    pub wrap mq::color::Color as Color {
        fields { r: f32, g: f32, b: f32, a: f32 }
        impl {
            pub const fn new(color: mq::color::Color) -> Self {
                Self(color)
            }
        }
        UserData {
            constructors {
                new (r g b a),
                from_rgba (r g b a),
                from_hex (hex),
                default ()
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
                new (x y)
            }
        }
    }

    #[derive(Default)]
    pub wrap mq::shapes::DrawRectangleParams as DrawRectangleParams {
        fields { offset: Vec2, rotation: f32, color: Color }
        impl {
            pub const fn new(params: mq::shapes::DrawRectangleParams) -> Self {
                Self(params)
            }
        }
        UserData {}
    }
}
