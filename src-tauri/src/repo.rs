use std::error::Error as DynError;
use std::str;
use std::fmt;
use serde::{Serialize, Deserialize};


// =============================================================
// ==================== STRUCTS AND CONSTANTS ==================
// =============================================================
#[derive(Serialize, Deserialize)]
struct RepoConfig {
    username: String,
    url: String,
}

#[derive(Debug)]
struct RepoError(String);

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl DynError for RepoError {}

// =============================================================
// ========================== FUNCTIONS ========================
// =============================================================
pub fn get_repo_json_config() -> Result<String, Box<dyn DynError>> {
    let repo_config = get_repo_config()?;
    Ok(serde_json::to_string(&repo_config)?)
}

pub fn set_repo_json_config(json_config: &str) -> Result<(), Box<dyn DynError>> {
    set_repo_config(&(
        serde_json::from_str(json_config)?
    ))?;

    Ok(())
}

pub fn is_repo_initialized() -> bool {
    git::is_git_initialized()
}




// =============================================================
// =========================== PRIVATE =========================
// =============================================================
fn get_repo_config() -> Result<RepoConfig, Box<dyn DynError>> {
    let username = get_repo_config_param("username")?;
    let url = get_repo_config_param("url")?;

    let repo_config = RepoConfig {
        username: String::from(username),
        url: String::from(url),
    };

    Ok(repo_config)
}


fn get_repo_config_param(param: &str) -> Result<String, Box<dyn DynError>> {
    let cmd_output = match param {
        "username" => git::get_username()?,
        "url" => git::get_url()?,
        _ => return Err(
            Box::new(RepoError(format!("The parameter {param} was not found in the repo config")))
        ) 
    };

    Ok(String::from(str::from_utf8(&cmd_output.stdout)?))
}

fn set_repo_config(repo_config: &RepoConfig) -> Result<(), Box<dyn DynError>> {
    git::set_username(repo_config.username.as_str())?; 
    git::set_url(repo_config.url.as_str())?;

    Ok(())
}




mod git {
    use std::process::Output;    
    use std::path::Path;
    use std::io::Error;

    use crate::ioutils::terminal;

    const REPO_PATH: &str = "./mojang";

    // =========== GIT REPOSITORY PATH FUNCTIONS ===========
    pub fn is_git_initialized() -> bool {
        Path::new(format!("{REPO_PATH}/.git").as_str()).exists()
    }

    // =========== GIT LOCAL CONFIG OPERATIONS ===========
    pub fn get_username() -> Result<Output, Error> {
        execute_git_command("config", vec!["--local", "user.name"])
    }

    pub fn set_username(username: &str) -> Result<Output, Error> {
        execute_git_command("config", vec!["--local", "user.name", username])
    }

    pub fn get_url() -> Result<Output, Error> {
        execute_git_command("ls-remote", vec!["--get-url"])
    }

    pub fn set_url(url: &str) -> Result<Output, Error> {
        execute_git_command("remote", vec!["set-url", "origin", url])
    }

    // =========== PRIVATE ===========
    fn execute_git_command<'a>(command: &'a str, mut args: Vec<&'a str>) -> Result<Output, Error> { 
        let mut git_args: Vec<&str> = vec![command];
        git_args.append(&mut args);
        terminal::execute_command("git", git_args, REPO_PATH)
    }
}