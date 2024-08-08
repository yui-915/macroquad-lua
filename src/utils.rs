use std::path::{Path, PathBuf};

use mlua::prelude::*;

pub trait MoreLua {
    fn load_module(
        &self,
        name: &str,
        f: impl FnOnce(&Lua) -> LuaResult<LuaTable>,
    ) -> LuaResult<&Lua>;
}

impl MoreLua for mlua::Lua {
    fn load_module(
        &self,
        name: &str,
        f: impl FnOnce(&Lua) -> LuaResult<LuaTable>,
    ) -> LuaResult<&Self> {
        self.globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("loaded")?
            .set(name, f(self)?)?;
        Ok(self)
    }
}

#[allow(dead_code)]
pub fn list_all_paths(path: &Path) -> Vec<PathBuf> {
    let mut paths = vec![];
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            paths.extend(list_all_paths(&path));
        } else {
            paths.push(path);
        }
    }
    paths
}

#[macro_export]
macro_rules! add_lua_fn {
    ($lua:expr, $table:expr, $name:expr, $func:expr, $($arg:ident) *) => {
        #[allow(unused_parens)]
        $table.set($name, $lua.create_function(|_, ($($arg),*)| Ok($func($($arg),*)))?)?;
    };
}

#[macro_export]
macro_rules! add_lua_fns {
    ($lua:expr, $table:expr, [$(($name:expr, $func:ident, $($arg:ident) *)),*]) => {
        $(
            add_lua_fn!($lua, $table, $name, $func, $($arg) *);
        )*
    };
}

#[macro_export]
macro_rules! make_lua_fns_table {
    ($lua:expr, [$(($name:expr, $func:ident, $($arg:ident) *)),*]) => {
        {
            let  table = $lua.create_table()?;
            add_lua_fns!($lua, table, [$(($name, $func, $($arg) *)),*]);
            table
        }
    };
}

#[macro_export]
macro_rules! make_lua_fns_table_smol {
    ($lua:expr, $($name:ident $($arg:ident) *),*) => {
        make_lua_fns_table!($lua, [$((stringify!($name), $name, $($arg) *)),*])
    }
}

#[macro_export]
macro_rules! make_lua_table {
    ($lua:expr, [$(($name:expr, $value:expr)),*]) => {
        {
            let table = $lua.create_table()?;
            $(
                table.set($name, $value)?;
            )*
            table
        }
    };
}

#[macro_export]
macro_rules! make_lua_constants_table {
    ($lua:expr, $($name:expr) *) => {
        {
            let table = $lua.create_table()?;
            $(
                table.set(stringify!($name), $name)?;
            )*
            table
        }
    };
}

#[macro_export]
macro_rules! wrap_type {
    ($type:ty, $alias:ident) => {
        #[derive(Default)]
        struct $alias($type);
        impl From<$type> for $alias {
            fn from(a: $type) -> Self {
                Self(a)
            }
        }
        impl From<$alias> for $type {
            fn from(a: $alias) -> Self {
                a.0
            }
        }
        impl $alias {
            pub const fn new(a: $type) -> Self {
                Self(a)
            }
        }
    };
}

#[macro_export]
macro_rules! wrap_fn_lua {
    ($original:expr, $name:ident, $return:ty, $($arg:ident $type:ty),*) => {
        #[allow(clippy::too_many_arguments)]
        fn $name ($($arg: $type),*) -> LuaResult< $return > {
            Ok( $original ( $($arg.into()),* ) )
        }
    };
}

#[macro_export]
macro_rules! impl_userdata_feilds {
    ($($field:ident) *) => {
        fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
            $(
                fields.add_field_method_get(stringify!($field), |_, s| Ok(s.0.$field));
                fields.add_field_method_set(stringify!($field), |_, s, v| Ok(s.0.$field = v));
            )*
        }
    }
}

#[macro_export]
macro_rules! impl_userdata_feilds_complex {
    ($($field:ident $type:ident),*) => {
        fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
            $(
                fields.add_field_method_get(stringify!($field), |_, s| Ok($type ::from(s.0.$field)));
                fields.add_field_method_set(stringify!($field), |_, s, v: $type| Ok(s.0.$field = v.into()));
            )*
        }
    }
}

#[macro_export]
macro_rules! impl_new {
    ($struct:ty, $trait:ident, $($field:ident $type:ty),*) => {
        trait $trait {
            fn new($($field: $type),*) -> Self;
        }
        impl $trait for $struct {
            fn new($($field: $type),*) -> Self {
                Self { $($field: $field.into()),* }
            }
        }
    };
}

#[macro_export]
macro_rules! extend_lua_table {
    ($dest:ident, $($src:ident) *) => {
        $(
            $src.for_each(|k: String, v: LuaValue| $dest.set(k, v))?;
        )*
    };
}

#[macro_export]
macro_rules! lua_wrap_constructor_fn {
    ($methods:ident, $($original:ident)::*, $struct:ident, $name:ident, $($arg:ident) *) => {
        #[allow(unused_parens)]
        $methods.add_function(stringify!($name), |_, ($($arg),*)| {
            Ok($struct ::new( $($original)::*::$name ($($arg),*)))
        });
    };
}

#[macro_export]
macro_rules! lua_wrap_constructor_fns {
    ($methods:ident, $original:ident, $struct:ident, [$($name:ident $($arg:ident) *),*]) => {
        $(
            lua_wrap_constructor_fn!($methods, $original, $struct, $name, $($arg) *);
        )*
    };
    ($methods:ident, $original1:ident::$original2:ident, $struct:ident, [$($name:ident $($arg:ident) *),*]) => {
        $(
            lua_wrap_constructor_fn!($methods, $original1::$original2, $struct, $name, $($arg) *);
        )*
    };
    ($methods:ident, $original1:ident::$original2:ident::$original3:ident, $struct:ident, [$($name:ident $($arg:ident) *),*]) => {
        $(
            lua_wrap_constructor_fn!($methods, $original1::$original2::$original3, $struct, $name, $($arg) *);
        )*
    };
    ($methods:ident, $original1:ident::$original2:ident::$original3:ident::$original4:ident, $struct:ident, [$($name:ident $($arg:ident) *),*]) => {
        $(
            lua_wrap_constructor_fn!($methods, $original1::$original2::$original3::$original4, $struct, $name, $($arg) *);
        )*
    };
}

#[macro_export]
macro_rules! impl_from_lua {
    ($($original:ident)::*, $struct:ident, $($field:ident) *) => {
        impl FromLua<'_> for $struct {
            fn from_lua(lua_value: LuaValue<'_>, _lua: &Lua) -> LuaResult<Self> {
                match lua_value {
                    LuaValue::Table(t) => Ok($struct ::new( $($original)::*::new ($(t.get(stringify!($field))?),*))),
                    LuaValue::UserData(ud) => Ok($struct ::new( $($original)::*::new ($(ud.get(stringify!($field))?),*))),
                    t => Err(mlua::Error::FromLuaConversionError {
                        from: t.type_name(),
                        to: stringify!($struct),
                        message: Some(format!("expected a {}", stringify!($struct))),
                    }),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! reconst {
    ($struct:ident, $path:ident, $($name:ident) *) => {
        $(
            const $name: $struct = $struct ::new($path ::$name);
        )*
    };
    ($struct:ident, $path1:ident::$path2:ident, $($name:ident) *) => {
        $(
            const $name: $struct = $struct ::new($path1 ::$path2 ::$name);
        )*
    };
    ($struct:ident, $path1:ident::$path2:ident::$path3:ident, $($name:ident) *) => {
        $(
            const $name: $struct = $struct ::new($path1 ::$path2 ::$path3 ::$name);
        )*
    };
    ($struct:ident, $path1:ident::$path2:ident::$path3:ident::$path4:ident, $($name:ident) *) => {
        $(
            const $name: $struct = $struct ::new($path1 ::$path2 ::$path3 ::$path4 ::$name);
        )*
    };
}
