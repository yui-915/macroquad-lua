use mlua::prelude::*;
use std::collections::HashMap;

#[cfg(feature = "dev")]
use std::path::PathBuf;

#[cfg(feature = "dev")]
mod debug;
#[cfg(feature = "release")]
mod release;

pub struct LuaWrapper {
    #[cfg(feature = "dev")]
    rx: std::sync::mpsc::Receiver<Result<notify::Event, notify::Error>>,
    #[cfg(feature = "dev")]
    watcher: notify::PollWatcher,
    #[cfg(feature = "dev")]
    path: PathBuf,

    pub lua: mlua::Lua,
    loaded_files: HashMap<String, String>,
}

fn format_lua_filename(name: &str) -> String {
    format!("={}.lua", name.split('.').collect::<Vec<_>>().join("_"))
}

impl LuaWrapper {
    pub fn load_modules(&self) -> mlua::Result<()> {
        for (name, content) in &self.loaded_files {
            self.lua
                .globals()
                .get::<_, LuaTable>("package")?
                .get::<_, LuaTable>("preload")?
                .set(
                    name.as_str(),
                    self.lua
                        .load(content)
                        .set_name(format_lua_filename(name))
                        .into_function()?,
                )?;
        }
        Ok(())
    }

    pub fn unload_modules(&self) -> mlua::Result<()> {
        self.lua
            .globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("preload")?
            .clear()?;
        self.lua
            .globals()
            .get::<_, LuaTable>("package")?
            .get::<_, LuaTable>("loaded")?
            .clear()?;
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
