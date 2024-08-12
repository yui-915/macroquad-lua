use mlua::Value::Nil;

wrap_generics_for_lua! {
    pub wrap std::collections::HashSet<D> as HashSet<T, D> {
        fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
            let table = lua.create_table()?;
            for key in self.0 {
                table.set(T::from(key), true)?;
            }
            table.into_lua(lua)
        }
    }

    pub wrap std::option::Option<D> as Option<T, D> {
        fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
            match self.0 {
                Some(value) => T::from(value).into_lua(lua),
                None => Ok(Nil),
            }
        }
    }

    pub wrap std::vec::Vec<D> as Vec<T, D> {
        fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
            let table = lua.create_table()?;
            for value in self.0 {
                table.set(T::from(value), true)?;
            }
            table.into_lua(lua)
        }
    }
}
