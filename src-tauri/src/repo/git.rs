use std::process::Output;    
use std::io::Error;

use crate::ioutils::terminal;
use crate::ioutils::file;

pub const REPO_PATH: &str = "./mojang";

// =========== GIT SETUP ===========
pub fn init() -> Result<Output, Error> {
    execute_git_command("init", None)
} 

pub fn lfs_track(regex: &str) -> Result<Output, Error> {
    execute_git_command("lfs", Some(vec!["tract", format!("\"{regex}\"").as_str()]))
}

pub fn is_git_initialized() -> bool {
    file::exists(format!("{REPO_PATH}/.git").as_str())
}

// =========== GIT LOCAL CONFIG OPERATIONS ===========
pub fn get_username() -> Result<Output, Error> {
    execute_git_command("config", Some(vec!["--local", "user.name"]))
}

pub fn set_username(username: &str) -> Result<Output, Error> {
    execute_git_command("config", Some(vec!["--local", "user.name", username]))
}

pub fn get_url() -> Result<Output, Error> {
    execute_git_command("ls-remote", Some(vec!["--get-url"]))
}

pub fn set_url(url: &str) -> Result<Output, Error> {
    execute_git_command("remote", Some(vec!["set-url", "origin", url]))
}

pub fn add_origin(origin_url: &str) -> Result<Output, Error> {
    execute_git_command("remote", Some(vec!["add", "origin", origin_url]))
}

// =========== BRANCHING =========
pub fn checkout(branch: &str, force: bool) -> Result<Output, Error> {
    let mut args = vec![branch];
    if force { args.push("--force"); }
    execute_git_command("checkout", Some(args))
}

// =========== GET AND UPDATE STATE ===========
pub fn fetch() -> Result<Output, Error> {
    execute_git_command("fetch", None)
}

pub fn pull(ff: bool) -> Result<Output, Error> {
    let mut args = None;
    if !ff { args = Some(vec!["--no-ff"]); }
    execute_git_command("pull", args)
}

// =========== PUSH DATA =========
pub fn add(files: Vec<&str>) -> Result<Output, Error> {
    execute_git_command("add", Some(files))
}

pub fn commit(message: &str) -> Result<Output, Error> {
    execute_git_command("commit", Some(vec!["-m", message]))
}

pub fn push() -> Result<Output, Error> {
    execute_git_command("push", None)
}

// =========== PRIVATE ===========
fn execute_git_command<'a>(command: &'a str, mut args: Option<Vec<&'a str>>) -> Result<Output, Error> { 
    let mut git_args: Vec<&str> = vec![command];
    if let Some(mut args) = args {
        git_args.append(&mut args);
    }

    terminal::execute_command("git", git_args, REPO_PATH)
}