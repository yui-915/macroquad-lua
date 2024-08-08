use mlua::prelude::*;
use std::collections::HashMap;

// if you modify this one, also modify the one in Lua::new() (release profile)
#[cfg(debug_assertions)]
const LUA_SRC_PATH: &str = "game/src";

#[cfg(debug_assertions)]
use crate::utils::list_all_paths;
#[cfg(debug_assertions)]
use notify::Watcher;
#[cfg(debug_assertions)]
use std::path::PathBuf;

#[cfg(not(debug_assertions))]
use macroquad_lua_macros::embed_lua_files;

pub struct Lua {
    #[cfg(debug_assertions)]
    rx: std::sync::mpsc::Receiver<Result<notify::Event, notify::Error>>,

    #[cfg(debug_assertions)]
    watcher: notify::PollWatcher,

    pub lua: mlua::Lua,

    loaded_files: HashMap<String, String>,
}

impl Lua {
    #[cfg(debug_assertions)]
    pub fn new() -> LuaResult<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::poll::PollWatcher::new(tx, notify::Config::default()).unwrap();
        watcher
            .watch(
                &PathBuf::from(LUA_SRC_PATH),
                notify::RecursiveMode::Recursive,
            )
            .unwrap();
        Ok(Self {
            rx,
            watcher,
            lua: mlua::Lua::new(),
            loaded_files: HashMap::new(),
        })
    }

    #[cfg(not(debug_assertions))]
    pub fn new() -> LuaResult<Self> {
        Ok(Self {
            lua: mlua::Lua::new(),
            loaded_files: embed_lua_files!("game/src"),
        })
    }

    pub fn poll(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.watcher.poll().unwrap();
            if let Ok(_event) = self.rx.try_recv() {
                println!("Reloading modules...");
                self.unload_modules().unwrap();
                self.load_files();
                self.load_modules().unwrap();
            }
        }
    }

    #[cfg(debug_assertions)]
    pub fn load_files(&mut self) {
        let base_path = PathBuf::from(LUA_SRC_PATH);
        for path in &list_all_paths(&base_path) {
            let mut name = path
                .iter()
                .skip(base_path.iter().count())
                .take(path.iter().count() - base_path.iter().count())
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join(".");
            name.pop();
            name.pop();
            name.pop();
            name.pop();
            self.loaded_files
                .insert(name, std::fs::read_to_string(path).unwrap());
        }
    }

    #[cfg(debug_assertions)]
    pub fn unload_files(&mut self) {
        self.loaded_files.clear();
    }

    #[cfg(debug_assertions)]
    pub fn reload_files(&mut self) {
        self.unload_files();
        self.load_files();
    }

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
}
