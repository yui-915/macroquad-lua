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

const LUA_SRC_PATH: &str = "game/src";

impl LuaWrapper {
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
