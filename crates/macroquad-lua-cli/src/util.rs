use git2::{
    build::CheckoutBuilder, AnnotatedCommit, Error as GitError, FetchOptions, Reference, Remote,
    Repository,
};
use std::path::PathBuf;

#[macro_export]
macro_rules! try_throw {
    {$($e:expr)+; $m:literal} => {
        if let Err(e) = $($e)+ {
            eprintln!("{}: {}", $m, e);
            std::process::exit(1);
        }
    }
}

#[macro_export]
macro_rules! unwrap {
    ($v:ident) => {
        let $v = $v.unwrap();
    };
}

#[macro_export]
macro_rules! try_throw_else {
    {$($e:expr)+; $m:literal; $($e2:tt)+} => {
        if let Err(e) = $($e)+ {
            eprintln!("{}: {}", $m, e);
            std::process::exit(1);
        }
        $($e2)+
    }
}

fn do_fetch<'a>(
    repo: &'a Repository,
    refs: &[&str],
    remote: &'a mut Remote,
) -> Result<AnnotatedCommit<'a>, GitError> {
    let mut fo = FetchOptions::new();
    fo.depth(1);
    remote.fetch(refs, Some(&mut fo), None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    repo.reference_to_annotated_commit(&fetch_head)
}

fn fast_forward(
    repo: &Repository,
    lb: &mut Reference,
    rc: &AnnotatedCommit,
) -> Result<(), GitError> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(CheckoutBuilder::default().force()))
}

fn normal_merge(
    repo: &Repository,
    local: &AnnotatedCommit,
    remote: &AnnotatedCommit,
) -> Result<(), GitError> {
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
    repo: &'a Repository,
    remote_branch: &str,
    fetch_commit: AnnotatedCommit<'a>,
) -> Result<(), GitError> {
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
                    CheckoutBuilder::default()
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

pub fn pull(repo: &Repository) -> Result<(), GitError> {
    let remote_name = "origin";
    let remote_branch = "master";
    let mut remote = repo.find_remote(remote_name)?;
    let fetch_commit = do_fetch(repo, &[remote_branch], &mut remote)?;
    do_merge(repo, remote_branch, fetch_commit)
}

pub fn copy_dir_all(
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

pub fn fetch_options<'a>() -> FetchOptions<'a> {
    let mut fo = FetchOptions::new();
    fo.depth(1);
    fo
}

pub fn git_pull(local_path: PathBuf) {
    println!("Pulling latest changes");
    let repo = git2::Repository::open(local_path);
    try_throw_else! {
        repo;
        "Failed to open local repo";
        unwrap!(repo)
    }
    try_throw_else! {
        pull(&repo);
        "Failed to pull";
        println!("Finished pull");
    }
}

pub fn git_clone(local_path: PathBuf, remote_url: &str) {
    println!("Cloning {}", remote_url);
    try_throw_else! {
        git2::build::RepoBuilder::new()
            .fetch_options(fetch_options())
            .clone(remote_url, &local_path);
        "Failed to clone macroquad-lua repo";
        println!("Finished clone");
    }
}

pub struct CargoCommand {
    pub repo: PathBuf,
    pub subcommand: CargoSubcommand,
}

pub enum CargoSubcommand {
    Run {
        quite: bool,
        path: Option<PathBuf>,
    },
    Build {
        path: Option<PathBuf>,
        target: Option<String>,
    },
    Clean,
}

pub fn cargo(command: CargoCommand) {
    let mut cmd = std::process::Command::new("cargo");
    cmd.current_dir(command.repo);

    match command.subcommand {
        CargoSubcommand::Run { quite, ref path } => {
            cmd.args(["run", "--release"]);
            cmd.env(
                "MACROQUAD_LUA_GAME_SRC_PATH",
                path.as_ref()
                    .unwrap_or(&std::env::current_dir().unwrap())
                    .join("src"),
            );
            if quite {
                cmd.arg("--quiet");
            }
        }
        CargoSubcommand::Build {
            ref path,
            ref target,
        } => {
            cmd.args(["build", "--release", "--no-default-features"]);
            cmd.env(
                "MACROQUAD_LUA_GAME_SRC_PATH",
                path.as_ref()
                    .unwrap_or(&std::env::current_dir().unwrap())
                    .join("src"),
            );

            if let Some(target) = target {
                cmd.arg("--target").arg(target);
                if target == "wasm32-unknown-emscripten" {
                    cmd.args(["--features", "release-nojit"]);
                } else {
                    cmd.args(["--features", "release-jit"]);
                }
            } else {
                cmd.args(["--features", "release-jit"]);
            }
        }
        CargoSubcommand::Clean => {
            cmd.arg("clean");
        }
    }

    let status = cmd.status();
    try_throw_else! {
        status;
        "Failed to build";
        unwrap!(status)
    }
    if !status.success() {
        eprintln!(
            "Failed to {}: {}",
            if let CargoSubcommand::Clean = command.subcommand {
                "clean"
            } else {
                "build"
            },
            status
        );
        std::process::exit(1);
    }
}

pub fn get_files_to_copy_list(
    repo: PathBuf,
    path: Option<PathBuf>,
    target: Option<String>,
) -> Vec<(PathBuf, PathBuf)> {
    let game_dir = path.unwrap_or(std::env::current_dir().unwrap());
    let build = game_dir.join("build");
    let name = game_dir.file_name().unwrap().to_string_lossy();

    match target {
        None => {
            #[cfg(target_os = "windows")]
            let filename = "macroquad-lua.exe";
            #[cfg(not(target_os = "windows"))]
            let filename = "macroquad-lua";

            vec![(
                repo.join("target").join("release").join(filename),
                build.join(filename.to_string().replace("macroquad-lua", &name)),
            )]
        }
        Some(target) => {
            if target == "android" {
                todo!()
            } else if target == "wasm32-unknown-emscripten" {
                let in_ = repo
                    .join("target")
                    .join("wasm32-unknown-emscripten")
                    .join("release");
                let out = build.join("wasm32-unknown-emscripten");
                vec![
                    (
                        in_.join("macroquad_lua.wasm"),
                        out.join("macroquad_lua.wasm"),
                    ),
                    (in_.join("macroquad-lua.js"), out.join("macroquad-lua.js")),
                    (repo.join("web").join("index.html"), out.join("index.html")),
                ]
            } else {
                let filename = if target.contains("windows") {
                    "macroquad-lua.exe"
                } else {
                    "macroquad-lua"
                };
                vec![(
                    repo.join("target")
                        .join(&target)
                        .join("release")
                        .join(filename),
                    build
                        .join(target)
                        .join(filename.to_string().replace("macroquad-lua", &name)),
                )]
            }
        }
    }
}
