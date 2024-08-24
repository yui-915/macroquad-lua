use clap::{Parser, Subcommand};
use platform_dirs::AppDirs;

#[macro_use]
mod util;
use util::*;

const REMOTE_URL: &str = "https://github.com/yui-915/macroquad-lua.git";

#[derive(Parser, Debug)]
#[command(version, about)]
/// A simple cli for macroquad-lua
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Update the internal macroquad-lua
    Update {
        /// Alternative remote repository url
        #[arg(long)]
        remote: Option<String>,
    },

    /// Reset macroquad-lua-cli (completely delete macroquad-lua)
    Reset,

    /// Run a macroquad-lua game
    Run {
        /// Suppress cargo output
        #[arg(long, short, default_value_t = false)]
        quite: bool,

        /// Path of the game's folder (optional, defaults to current directory)
        #[arg()]
        path: Option<std::path::PathBuf>,

        /// Alternative local repository path (macroquad-lua)
        #[arg(long)]
        repo: Option<std::path::PathBuf>,
    },

    /// Initialize a new macroquad-lua project
    Init {
        /// Path of the game's folder (optional, defaults to current directory)
        #[arg()]
        path: Option<std::path::PathBuf>,
    },

    /// Create a new macroquad-lua project
    New {
        /// Path of the game's folder
        #[arg()]
        path: std::path::PathBuf,
    },

    /// Build a macroquad-lua project
    Build {
        /// Path of the game's folder (optional, defaults to current directory)
        #[arg()]
        path: Option<std::path::PathBuf>,

        /// Build target (either triple or emscripten/web/wasm/android, optional)
        #[arg(long)]
        target: Option<String>,

        /// Alternative local repository path (macroquad-lua)
        #[arg(long)]
        repo: Option<std::path::PathBuf>,
    },

    /// Run `cargo clean` on the internal macroquad-lua repo
    Clean,
}

fn main() {
    let cli = Cli::parse();
    let app_dirs = AppDirs::new(Some("macroquad-lua-cli"), true).unwrap();
    let macroquad_lua_repo_path = app_dirs.data_dir.join("macroquad-lua");

    std::fs::create_dir_all(&app_dirs.data_dir).unwrap();

    // TODO: disable this check if a --repo argument is provided
    if !macroquad_lua_repo_path.exists() {
        match cli.command {
            Commands::Update { .. } => (),
            _ => {
                eprintln!("macroquad-lua is not installed");
                eprintln!("Please run `mql update`");
                std::process::exit(1);
            }
        }
    }

    match cli.command {
        Commands::Update { remote } => {
            println!("Note: this command only updates macroquad-lua, not macroquad-lua-cli");

            let local_path = macroquad_lua_repo_path;
            let remote_url = remote.unwrap_or(REMOTE_URL.to_string());

            if !local_path.exists() {
                git_clone(local_path, &remote_url);
            } else {
                git_pull(local_path);
            }
            println!("Up to date");
        }

        Commands::Reset => {
            println!("Resetting macroquad-lua-cli");
            if macroquad_lua_repo_path.exists() {
                try_throw_else! {
                    std::fs::remove_dir_all(macroquad_lua_repo_path);
                    "Failed to delete macroquad-lua repo";
                    println!("Deleted macroquad-lua repo")
                }
            }
            println!("Done");
        }

        Commands::Init { path } => {
            let src = macroquad_lua_repo_path.join("game");
            let dst = path.unwrap_or(std::env::current_dir().unwrap());
            try_throw_else! {
                copy_dir_all(src, dst);
                "Failed to copy game template";
                println!("Done")
            }
        }

        Commands::New { path } => {
            let src = macroquad_lua_repo_path.join("game");
            let dst = path;
            try_throw! {
                std::fs::create_dir_all(&dst);
                "Failed to create game folder"
            }
            try_throw_else! {
                copy_dir_all(src, dst);
                "Failed to copy game template";
                println!("Done")
            }
        }

        Commands::Run { quite, path, repo } => {
            cargo(CargoCommand {
                repo: repo.unwrap_or(macroquad_lua_repo_path),
                subcommand: CargoSubcommand::Run { quite, path },
            });
        }

        Commands::Build { path, target, repo } => {
            let repo = repo.unwrap_or(macroquad_lua_repo_path.clone());
            let target = target.map(|target| match target.as_str() {
                "emscripten" | "web" | "wasm" => "wasm32-unknown-emscripten".to_string(),
                _ => target,
            });
            if target.is_some() && target.as_ref().unwrap() == "android" {
                println!("Building for android is not supported yet");
                std::process::exit(1);
            }
            cargo(CargoCommand {
                repo: repo.clone(),
                subcommand: CargoSubcommand::Build {
                    target: target.clone(),
                    path: path.clone(),
                },
            });

            for (src, dst) in get_files_to_copy_list(repo, path, target) {
                try_throw! {
                    std::fs::create_dir_all(dst.parent().unwrap());
                    "Failed to create game folder"
                }
                try_throw! {
                    std::fs::copy(src, dst);
                    "Failed to copy compiled game"
                }
            }

            println!("Done");
        }
        Commands::Clean => {
            cargo(CargoCommand {
                repo: macroquad_lua_repo_path,
                subcommand: CargoSubcommand::Clean,
            });
        }
    }
}
