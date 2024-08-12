use mlua::prelude::*;
use std::collections::HashMap;

#[cfg(debug_assertions)]
mod debug;
#[cfg(not(debug_assertions))]
mod release;

pub struct LuaWrapper {
    #[cfg(debug_assertions)]
    rx: std::sync::mpsc::Receiver<Result<notify::Event, notify::Error>>,
    #[cfg(debug_assertions)]
    watcher: notify::PollWatcher,

    pub lua: mlua::Lua,
    loaded_files: HashMap<String, String>,
}

impl LuaWrapper {
    pub fn load_modules(&self) -> mlua::Result<()> {
        for (name, content) in &self.loaded_files {
            self.lua
                .globals()
                .get::<_, LuaTable>("package")?
                .get::<_, LuaTable>("preload")?
                .set(name.as_str(), self.lua.load(content).into_function()?)?;
        }
        let main: LuaTable = self
            .lua
            .globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("preload")?
            .get::<_, LuaFunction>("main")?
            .call(())?;
        self.lua
            .globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("loaded")?
            .set("main", main)?;
        Ok(())
    }

    pub fn unload_modules(&self) -> mlua::Result<()> {
        self.lua
            .globals()
            .get::<_, LuaTable>("package")?
            .set("preload", self.lua.create_table()?)?;
        self.lua
            .globals()
            .get::<_, LuaTable>("package")?
            .set("loaded", self.lua.create_table()?)?;
        Ok(())
    }

    pub fn load_module(
        &self,
        name: &str,
        f: impl FnOnce(&Lua) -> LuaResult<LuaTable>,
    ) -> LuaResult<&Self> {
        self.lua
            .globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("loaded")?
            .set(name, f(&self.lua)?)?;
        Ok(self)
    }
}
