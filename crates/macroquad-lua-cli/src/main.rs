use clap::{Parser, Subcommand};
use platform_dirs::AppDirs;

const REPO_URL: &str = "https://github.com/yui-915/macroquad-lua.git";

#[derive(Parser, Debug)]
#[command(version, about)]
/// A simple cli for macroquad-lua
struct Cli {
    #[command(subcommand, name = "subcommand")]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Update the internal macroquad-lua
    Update,
    /// Reset macroquad-lua-cli (completely delete macroquad-lua)
    Reset,
    /// Run a macroquad-lua game from the current directory
    Run {
        /// Suppress cargo output
        #[arg(long, short, default_value_t = false)]
        quite: bool,

        /// Run in release mode
        #[arg(long, default_value_t = false)]
        release: bool,

        /// Path of the game's folder (optional, defaults to current directory)
        #[arg()]
        path: Option<std::path::PathBuf>,
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

        /// Build target (either triple or "emscripten"/"web"/"wasm" or "android", optional, defaults to current os)
        #[arg(long)]
        target: Option<String>,
    },
    /// Run `cargo clean` on the internal macroquad-lua repo
    Clean,
}

fn main() {
    let cli = Cli::parse();
    let app_dirs = AppDirs::new(Some("macroquad-lua-cli"), true).unwrap();
    let macroquad_lua_repo_path = app_dirs.data_dir.join("macroquad-lua");

    std::fs::create_dir_all(&app_dirs.data_dir).unwrap();

    if !macroquad_lua_repo_path.exists() {
        match cli.command {
            Commands::Update => (),
            _ => {
                eprintln!("macroquad-lua is not installed");
                eprintln!("Please run `mql update`");
                std::process::exit(1);
            }
        }
    }

    match cli.command {
        Commands::Update => {
            println!(
                "Please note that this command only updates macroquad-lua, not macroquad-lua-cli"
            );
            if !macroquad_lua_repo_path.exists() {
                println!("Cloning {}", REPO_URL);
                let mut fo = git2::FetchOptions::new();
                fo.depth(1);
                if let Err(e) = git2::build::RepoBuilder::new()
                    .fetch_options(fo)
                    .clone(REPO_URL, &macroquad_lua_repo_path)
                {
                    eprintln!("Failed to clone macroquad-lua repo: {}", e);
                    std::process::exit(1);
                }
                println!("Finished clone");
            } else {
                println!("Pulling latest changes");

                let repo = git2::Repository::open(macroquad_lua_repo_path);
                if let Err(e) = repo {
                    eprintln!("Failed to open macroquad-lua repo: {}", e);
                    eprintln!(
                        "Please try again (run `mql reset` to delete the repo incase it keeps failing)"
                    );
                    std::process::exit(1);
                }
                let repo = repo.unwrap();
                if let Err(e) = pull(&repo) {
                    eprintln!("Failed to pull: {}", e);
                    eprintln!(
                        "Please try again (run `mql reset` to delete the repo incase it keeps failing)"
                    );
                    std::process::exit(1);
                }
            }
            println!("Up to date");
        }
        Commands::Reset => {
            println!("Resetting macroquad-lua-cli");
            if macroquad_lua_repo_path.exists() {
                if let Err(e) = std::fs::remove_dir_all(macroquad_lua_repo_path) {
                    eprintln!("Failed to delete macroquad-lua repo: {}", e);
                    std::process::exit(1);
                }
                println!("Deleted macroquad-lua repo");
            }
        }
        Commands::Run {
            quite,
            release,
            path,
        } => {
            // let manifest_path = macroquad_lua_repo_path.join("Cargo.toml");
            // let manifest_path = manifest_path.to_str().unwrap();
            let game_src = path.unwrap_or(std::env::current_dir().unwrap()).join("src");
            let mut cmd = std::process::Command::new("cargo");
            cmd.current_dir(macroquad_lua_repo_path);

            cmd.arg("run");
            // cmd.args(["--manifest-path", manifest_path]);
            cmd.env("MACROQUAD_LUA_GAME_SRC_PATH", game_src);

            if quite {
                cmd.arg("--quiet");
            }
            if release {
                cmd.arg("--release");
            }

            let status = cmd.status();
            if let Err(e) = status {
                eprintln!("Failed to build: {}", e);
                std::process::exit(1);
            }
            let status = status.unwrap();
            if !status.success() {
                eprintln!("Failed to build: {}", status);
                std::process::exit(1);
            }
            std::process::exit(1);
        }
        Commands::Init { path } => {
            let src = macroquad_lua_repo_path.join("game");
            let dst = path.unwrap_or(std::env::current_dir().unwrap());
            if let Err(e) = copy_dir_all(src, dst) {
                eprintln!("Failed to copy game template: {}", e);
                std::process::exit(1);
            }
            println!("Done");
        }
        Commands::New { path } => {
            let src = macroquad_lua_repo_path.join("game");
            let dst = path;
            if let Err(e) = std::fs::create_dir_all(&dst) {
                eprintln!("Failed to create game folder: {}", e);
                std::process::exit(1);
            }
            if let Err(e) = copy_dir_all(src, dst) {
                eprintln!("Failed to copy game template: {}", e);
                std::process::exit(1);
            }
            println!("Done");
        }
        Commands::Build { path, target } => {
            // let manifest_path = macroquad_lua_repo_path.join("Cargo.toml");
            // let manifest_path = manifest_path.to_str().unwrap();
            let game_dir = path.unwrap_or(std::env::current_dir().unwrap());
            let game_src = game_dir.join("src");
            let mut cmd = std::process::Command::new("cargo");
            cmd.current_dir(&macroquad_lua_repo_path);

            let target = target.map(|target| match target.as_str() {
                "emscripten" | "web" | "wasm" => "wasm32-unknown-emscripten".to_string(),
                _ => target,
            });

            if target.is_some() && target.as_ref().unwrap().as_str() == "android" {
                panic!("Android support is not added yet")
            } else {
                cmd.arg("build");
                cmd.arg("--release");
                // cmd.args(["--manifest-path", manifest_path]);
                cmd.env("MACROQUAD_LUA_GAME_SRC_PATH", game_src);

                if let Some(target) = target.as_ref() {
                    cmd.arg("--target").arg(target);
                }
            }

            let status = cmd.status();
            if let Err(e) = status {
                eprintln!("Failed to build: {}", e);
                std::process::exit(1);
            }
            let status = status.unwrap();
            if !status.success() {
                eprintln!("Failed to build: {}", status);
                std::process::exit(1);
            }

            let game_build = game_dir.join("build");
            std::fs::create_dir_all(&game_build).unwrap();

            if target.is_none() {
                if let Err(e) = std::fs::copy(
                    macroquad_lua_repo_path
                        .join("target")
                        .join("release")
                        .join({
                            #[cfg(target_os = "windows")]
                            {
                                "macroquad-lua.exe"
                            }
                            #[cfg(not(target_os = "windows"))]
                            {
                                "macroquad-lua"
                            }
                        }),
                    game_build.join({
                        #[cfg(target_os = "windows")]
                        {
                            (game_dir.file_name().unwrap().to_string_lossy() + ".exe").to_string()
                        }
                        #[cfg(not(target_os = "windows"))]
                        {
                            game_dir.file_name().unwrap()
                        }
                    }),
                ) {
                    eprintln!("Failed to copy compiled game: {}", e);
                    std::process::exit(1);
                }
            } else {
                let target = target.unwrap();
                let path = match target.as_str() {
                    "android" => todo!(),
                    _ => macroquad_lua_repo_path
                        .join("target")
                        .join(&target)
                        .join("release"),
                };
                let dst = game_build.join(&target);
                std::fs::create_dir_all(&dst).unwrap();

                if target.as_str() == "wasm32-unknown-emscripten" {
                    if let Err(e) = std::fs::copy(
                        path.join("macroquad_lua.wasm"),
                        dst.join("macroquad_lua.wasm"),
                    ) {
                        eprintln!("Failed to copy compiled game: {}", e);
                        std::process::exit(1);
                    }
                    if let Err(e) =
                        std::fs::copy(path.join("macroquad-lua.js"), dst.join("macroquad-lua.js"))
                    {
                        eprintln!("Failed to copy compiled game: {}", e);
                        std::process::exit(1);
                    }
                    if let Err(e) = std::fs::copy(
                        macroquad_lua_repo_path.join("web").join("index.html"),
                        dst.join("index.html"),
                    ) {
                        eprintln!("Failed to copy compiled game: {}", e);
                        std::process::exit(1);
                    }
                } else if target.split("windows").collect::<Vec<_>>().len() == 2 {
                    if let Err(e) = std::fs::copy(
                        path.join("macroquad-lua.exe"),
                        dst.join(
                            (game_dir.file_name().unwrap().to_string_lossy() + ".exe").to_string(),
                        ),
                    ) {
                        eprintln!("Failed to copy compiled game: {}", e);
                        std::process::exit(1);
                    }
                } else if let Err(e) = std::fs::copy(
                    path.join("macroquad-lua"),
                    dst.join(game_dir.file_name().unwrap()),
                ) {
                    eprintln!("Failed to copy compiled game: {}", e);
                    std::process::exit(1);
                }
                println!("Done");
            }
        }
        Commands::Clean => {
            let mut cmd = std::process::Command::new("cargo");
            cmd.current_dir(macroquad_lua_repo_path);

            cmd.arg("clean");

            let status = cmd.status();
            if let Err(e) = status {
                eprintln!("Failed to build: {}", e);
                std::process::exit(1);
            }
            let status = status.unwrap();
            if !status.success() {
                eprintln!("Failed to build: {}", status);
                std::process::exit(1);
            }
            std::process::exit(1);
        }
    }
}

fn copy_dir_all(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn do_fetch<'a>(
    repo: &'a git2::Repository,
    refs: &[&str],
    remote: &'a mut git2::Remote,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
    let mut fo = git2::FetchOptions::new();
    // fo.download_tags(git2::AutotagOption::All);
    fo.depth(1);
    remote.fetch(refs, Some(&mut fo), None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    repo.reference_to_annotated_commit(&fetch_head)
}

fn fast_forward(
    repo: &git2::Repository,
    lb: &mut git2::Reference,
    rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
}

fn normal_merge(
    repo: &git2::Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    let msg = format!("Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;
    repo.checkout_head(None)
}

fn do_merge<'a>(
    repo: &'a git2::Repository,
    remote_branch: &str,
    fetch_commit: git2::AnnotatedCommit<'a>,
) -> Result<(), git2::Error> {
    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", remote_branch);
        match repo.find_reference(&refname) {
            Ok(mut r) => fast_forward(repo, &mut r, &fetch_commit),
            Err(_) => {
                repo.reference(
                    &refname,
                    fetch_commit.id(),
                    true,
                    &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
                )?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(
                    git2::build::CheckoutBuilder::default()
                        .allow_conflicts(true)
                        .conflict_style_merge(true)
                        .force(),
                ))
            }
        }
    } else if analysis.0.is_normal() {
        let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
        normal_merge(repo, &head_commit, &fetch_commit)
    } else {
        Ok(())
    }
}

fn pull(repo: &git2::Repository) -> Result<(), git2::Error> {
    let remote_name = "origin";
    let remote_branch = "master";
    let mut remote = repo.find_remote(remote_name)?;
    let fetch_commit = do_fetch(repo, &[remote_branch], &mut remote)?;
    do_merge(repo, remote_branch, fetch_commit)
}
