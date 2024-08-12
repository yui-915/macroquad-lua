#[macro_export]
macro_rules! wrap_enums_for_lua {
    {$(
        $visibility:vis wrap $original:path as $new:ident {
            $($variant:ident) *
        }
    )*} => {$(
        $visibility enum $new {
            $($variant),*
        }

        impl From<$original> for $new {
            fn from(a: $original) -> Self {
                use $original as Original;
                match a {
                    $(Original::$variant => Self::$variant,)*
                }
            }
        }

        impl From<$new> for $original {
            fn from(a: $new) -> Self {
                use $new as New;
                match a {
                    $(New::$variant => Self::$variant,)*
                }
            }
        }

        impl FromLua<'_> for $new {
            fn from_lua(lua_value: LuaValue<'_>, _lua: &Lua) -> LuaResult<Self> {
                match lua_value {
                    LuaValue::String(s) => match s.to_str()? {
                        $(stringify!($variant) => Ok(Self::$variant),)*
                        _ => Err(mlua::Error::FromLuaConversionError {
                            from: "String",
                            to: stringify!($new),
                            message: Some(format!(
                                "unable to convert {} from lua into {}",
                                "String",
                                stringify!($new)
                            ))
                        }),                        },
                    t => Err(mlua::Error::FromLuaConversionError {
                            from: t.type_name(),
                            to: stringify!($new),
                            message: Some(format!(
                                "unable to convert {} from lua into {}",
                                t.type_name(),
                                stringify!($new)
                            ))
                        }),
                }
            }
        }

        impl IntoLua<'_> for $new {
            fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
                match self {
                    $($new::$variant => lua.create_string(stringify!($variant))?,)*
                }
                .into_lua(lua)
            }
        }
    )*}
}

#[macro_export]
macro_rules! wrap_structs_for_lua {
    {$(
        $(#[$($attr:meta),*])?
        $visibility:vis wrap $original:path as $new:ident {

            fields {
                $($field_visibility:vis $field_name:ident : $field_type:ty),*
            }

            $(
                impl $impl:tt
            )?

            UserData {
                $(
                    constructors {
                        $($constructor_name:ident($($constructor_arg:ident) *)),*
                    }
                )?
                $(
                    impl ($methods:ident: &mut LuaUserDataMethods) $methods_body:tt
                )?
            }

            $(
                $constants_visibility:vis constants from $constants_source:ident {
                    $($constant_name:ident) *
                }
            )?
        }
    )*} => {$(
        $(#[$($attr),*])?
        $visibility struct $new($original);

        impl From<$original> for $new {
            fn from(a: $original) -> Self {
                Self(a)
            }
        }

        impl From<$new> for $original {
            fn from(a: $new) -> Self {
                a.0
            }
        }

        impl mlua::FromLua<'_> for $new {
            fn from_lua(lua_value: mlua::Value<'_>, _lua: &mlua::Lua) -> mlua::Result<Self> {
                use $original as Original;
                use mlua::AnyUserDataExt;
                match lua_value {
                    mlua::Value::Table(t) => Ok(
                        Self (
                            Original {
                                $($field_name: t.get::<_, $field_type>(stringify!($field_name))?.into()),*
                            }
                        )
                     ),
                    mlua::Value::UserData(ud) => Ok(
                        Self (
                            Original {
                                $($field_name: ud.get::<_, $field_type>(stringify!($field_name))?.into()),*
                            }
                        )
                     ),
                    t => Err(mlua::Error::FromLuaConversionError {
                            from: t.type_name(),
                            to: stringify!($new),
                            message: Some(format!(
                                "unable to convert {} from lua into {}",
                                t.type_name(),
                                stringify!($new)
                            ))
                        }),
                }
            }
        }

        $(
            impl $new $impl
        )?

        impl mlua::UserData for $new {
            fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
                $({
                    use $field_type as FieldType;
                    fields.add_field_method_get(
                        stringify!($field_name),
                        |_, s| Ok(FieldType::from(s.0.$field_name))
                    );
                    fields.add_field_method_set(
                        stringify!($field_name),
                        |_, s, v: FieldType| Ok(s.0.$field_name = v.into())
                    );
                })*
            }
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(__methods__: &mut M) {
                $($({
                    use $original as Original;
                    #[allow(unused_parens)]
                    __methods__.add_function(
                        stringify!($constructor_name),
                        |_, ($($constructor_arg),*)| Ok(
                            Self( Original::$constructor_name ($($constructor_arg),*))
                        )
                    );
                })*)?
                $(
                    let $methods = __methods__;
                    $methods_body
                )?
            }
        }

        $($(
            $constants_visibility const $constant_name: $new = $new($constants_source::$constant_name);
        )*)?

    )*}
}

#[macro_export]
macro_rules! wrap_generics_for_lua {
    {$(
        $visibility:vis wrap $original:path as $new:ident <$T:ident, $D:ident> {
            $(
                fn into_lua($into_lua_self:ident, $into_lua_lua:ident: &Lua) -> LuaResult<LuaValue> $into_lua_body:tt
            )?
        }
    )*} => {$(
        $visibility struct $new<$T, $D>($original, std::marker::PhantomData<$T>);

        impl<$T, $D> From<$new<$T, $D>> for $original {
            fn from(wrapped: $new<$T, $D>) -> Self {
                wrapped.0
            }
        }

        impl<$T, $D> From<$original> for $new<$T, $D> {
            fn from(non_wrapped: $original) -> Self {
                Self(non_wrapped, std::marker::PhantomData)
            }
        }


        $(
            impl<'lua, $T: From<$D> + mlua::IntoLua<'lua>, $D> mlua::IntoLua<'lua> for $new<$T, $D> {
                fn into_lua($into_lua_self, $into_lua_lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value> $into_lua_body
            }
        )?
    )*}
}

#[macro_export]
macro_rules! wrap_functions_for_lua {
    {$(
        $(#[$($attr:meta),*])?
        $visibility:vis wrap $original:path as $new:ident ($($arg_name:ident: $arg_type:ty),*) -> $return:ty
    )*} => {$(
        $(#[$($attr),*])?
        $visibility fn $new ($($arg_name: $arg_type),*) -> $return {
            $original ( $($arg_name.into()),* ).into()
        }
    )*};
}

#[macro_export]
macro_rules! lua_table {
    {
        using $lua:ident;
        $(
            extends $($extention_table:expr),*;
        )?

        $(
            functions {
               $($func:ident ($($arg:ident) *)),*
            }
        )?

        $(
            fields {
                $($field_name:ident : $field_value:expr),*
            }
        )?

        $(
            constants {
                $($constant:expr) *
            }
        )?

        $(
            enums {
                $($enum_name:ident { $($enum_variant:ident) * }),*
            }
        )?
    } => {
        {
            let table = $lua.create_table()?;
            $($(
                $extention_table.for_each(|k: String, v: LuaValue| table.set(k, v))?;
            )*)?
            $($(
                #[allow(unused_parens)]
                table.set(stringify!($func), $lua.create_function(|_, ($($arg),*)| Ok($func($($arg),*)))?)?;
            )*)?
            $($(
                table.set(stringify!($field_name), $field_value)?;
            )*)?
            $($(
                table.set(stringify!($constant), $constant)?;
            )*)?
            $($(
                table.set(stringify!($enum_name), lua_table! {
                    using $lua;
                    fields {
                        $($enum_variant: stringify!($enum_variant)),*
                    }
                })?;
            )*)?
            table
        }
    };
}
