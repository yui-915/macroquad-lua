mod lua;

use lua::LuaWrapper;
mod macroquad_bindings;
use macroquad::prelude::*;
use mlua::prelude::*;

macro_rules! get_err {
    ($($result:tt)+) => {
        if let Some(Err(e)) = $($result)+ {
            Some(e)
        } else {
            None
        }
    };
}

macro_rules! call_fn {
    ($lua:ident $table:ident.$fn:ident($($arg:expr),*) -> $ret:ty) => {
        match get_fn!($lua $table.$fn) {
            Some(ref f) => Some(f.call::<_, $ret>($($arg),*)),
            None => None,
        }
    };
}

macro_rules! get_fn {
    ($lua:ident $table:ident.$name:ident) => {
        match $lua
            .lua
            .globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("loaded")?
            .get::<_, LuaTable>(stringify!($table))?
            .get::<_, LuaFunction>(stringify!($name))
        {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    };
}

macro_rules! update_err {
    ($err:ident $lua:ident $table:ident.$fn:ident($($arg:expr),*) -> $ret:ty) => {
        $err = get_err!(call_fn!($lua $table.$fn($($arg),*) -> $ret))
    }
}

macro_rules! inline_if {
    ($condition:expr, $($code:tt)*) => {
        if $condition {
            $($code)*
        }
    };
}

macro_rules! load {
    ($lua:ident $modname:ident) => {
        $lua.lua
            .globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("loaded")?
            .set(
                stringify!($modname),
                $lua.lua
                    .globals()
                    .get::<_, LuaTable>("package")?
                    .get::<_, LuaTable>("preload")?
                    .get::<_, LuaFunction>(stringify!($modname))?
                    .call::<_, LuaValue>(())?,
            )
    };
}

#[macroquad::main("test")]
async fn main() -> LuaResult<()> {
    #[cfg(feature = "dev")]
    {
        let mut lua = LuaWrapper::new()?;
        lua.load_files();
        lua.load_modules()?;
        lua.load_module("macroquad", macroquad_bindings::module)?;
        load!(lua main)?;

        let mut last_err: Option<LuaError> = get_err!(call_fn!(lua macroquad.init(()) -> ()));
        if last_err.is_none() {
            update_err!(last_err lua macroquad.load(()) -> ())
        };

        if get_fn!(lua macroquad.update).is_none() && get_fn!(lua macroquad.draw).is_none() {
            if let Some(err) = last_err {
                eprintln!("{err:#?}");
            }
            return Err(LuaError::external("no update or draw function found"));
        }

        loop {
            // MAIN LOOP
            loop {
                inline_if!(last_err.is_some(), break);

                if lua.poll() {
                    lua.load_module("macroquad", macroquad_bindings::module)?;
                    load!(lua main)?;
                    update_err!(last_err lua macroquad.load(()) -> ());
                    inline_if!(last_err.is_some(), break);
                }

                update_err!(last_err lua macroquad.update(()) -> ());
                inline_if!(last_err.is_some(), break);

                update_err!(last_err lua macroquad.draw(()) -> ());
                next_frame().await;
            }
            // PANIC LOOP
            if get_fn!(lua macroquad.panic).is_none() || get_fn!(lua macroquad.panic_draw).is_none()
            {
                return Err(last_err.unwrap());
            }
            update_err!(last_err lua macroquad.panic(last_err.as_ref().unwrap().to_string()) -> ());
            inline_if!(last_err.is_some(), return Err(last_err.unwrap()));
            loop {
                update_err!(last_err lua macroquad.panic_draw(()) -> ());
                inline_if!(last_err.is_some(), return Err(last_err.unwrap()));

                if lua.poll() {
                    lua.load_module("macroquad", macroquad_bindings::module)?;
                    load!(lua main)?;
                    update_err!(last_err lua macroquad.load(()) -> ());
                    break;
                }
                next_frame().await;
            }
        }
    }

    #[cfg(feature = "release")]
    {
        let lua = LuaWrapper::new()?;
        lua.load_modules()?;
        lua.load_module("macroquad", macroquad_bindings::module)?;
        load!(lua main)?;

        if get_fn!(lua macroquad.update).is_none() && get_fn!(lua macroquad.draw).is_none() {
            return Err(LuaError::external("no update or draw function found"));
        }

        let mut last_err: Option<LuaError> = get_err!(call_fn!(lua macroquad.init(()) -> ()));
        if last_err.is_none() {
            update_err!(last_err lua macroquad.load(()) -> ())
        };

        // MAIN LOOP
        loop {
            inline_if!(last_err.is_some(), break);
            update_err!(last_err lua macroquad.update(()) -> ());
            inline_if!(last_err.is_some(), break);
            update_err!(last_err lua macroquad.draw(()) -> ());
            next_frame().await;
        }

        // PANIC LOOP
        if get_fn!(lua macroquad.panic).is_none() || get_fn!(lua macroquad.panic_draw).is_none() {
            return Err(last_err.unwrap());
        }
        update_err!(last_err lua macroquad.panic(last_err.as_ref().unwrap().to_string()) -> ());
        inline_if!(last_err.is_some(), return Err(last_err.unwrap()));
        loop {
            update_err!(last_err lua macroquad.panic_draw(()) -> ());
            inline_if!(last_err.is_some(), return Err(last_err.unwrap()));
            next_frame().await;
        }
    }
}
