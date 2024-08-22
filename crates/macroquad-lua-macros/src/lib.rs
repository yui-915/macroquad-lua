use quote::__private::TokenStream;
use quote::quote;
use std::path::Path;
use std::path::PathBuf;

#[proc_macro]
pub fn embed_lua_files(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let base_path_str = std::option_env!("MACROQUAD_LUA_GAME_SRC_PATH").unwrap_or("game/src");
    let base_path = Path::new(&base_path_str);
    let mut parts: Vec<TokenStream> = vec![];

    for path in &list_all_paths(base_path) {
        let mut name = path
            .iter()
            .skip(base_path.iter().count())
            .take(path.iter().count() - base_path.iter().count())
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
        let absolute_path = path.canonicalize().unwrap();
        let path_str = absolute_path.to_str().unwrap();
        parts.push(quote! {
            map.insert(#name.to_string(), include_str!(#path_str).to_string());
        });
    }
    quote! {
        {
            let mut map = std::collections::HashMap::new();
            #(#parts)*
            map
        }
    }
    .into()
}

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
