use super::{LuaResult, LuaWrapper};

use notify::Watcher;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn list_all_paths(path: &Path) -> Vec<PathBuf> {
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

impl LuaWrapper {
    pub fn new() -> LuaResult<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        let path = std::env::var("MACROQUAD_LUA_GAME_SRC_PATH").unwrap_or("game/src".to_string());
        let path = PathBuf::from(path);
        let mut watcher = notify::poll::PollWatcher::new(tx, notify::Config::default()).unwrap();
        watcher
            .watch(&path, notify::RecursiveMode::Recursive)
            .unwrap();
        Ok(Self {
            rx,
            watcher,
            path,
            lua: mlua::Lua::new(),
            loaded_files: HashMap::new(),
        })
    }

    pub fn load_files(&mut self) {
        for path in &list_all_paths(&self.path) {
            let mut name = path
                .iter()
                .skip(self.path.iter().count())
                .take(path.iter().count() - self.path.iter().count())
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join(".");
            if !name.ends_with(".lua") {
                continue;
            }
            name.pop();
            name.pop();
            name.pop();
            name.pop();
            self.loaded_files
                .insert(name, std::fs::read_to_string(path).unwrap());
        }
    }

    fn unload_files(&mut self) {
        self.loaded_files.clear();
    }

    fn reload_files(&mut self) {
        self.unload_files();
        self.load_files();
    }

    pub fn poll(&mut self) -> bool {
        self.watcher.poll().unwrap();
        if let Ok(_event) = self.rx.try_recv() {
            while let Ok(_event) = self.rx.try_recv() {}
            println!("Reloading modules...");
            self.unload_modules().unwrap();
            self.reload_files();
            self.load_modules().unwrap();
            true
        } else {
            false
        }
    }
}
