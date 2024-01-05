mod git;

use std::error::Error as DynError;
use std::str;
use std::fmt;

use serde::{Serialize, Deserialize};

use super::ioutils::file;

const HOSTFILE_PATH: &str = "/.host";


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
pub fn is_repo_initialized() -> bool {
    git::is_git_initialized()
}

pub fn init_repo(repo_config: &str) -> Result<(), Box<dyn DynError>> {
    // JSON string gets converted to RepoConfig object
    let repo_config: RepoConfig = serde_json::from_str(repo_config)?;

    // Repository is locally initialized and then configured
    git::init()?;
    git::set_username(repo_config.username.as_str())?;
    git::add_origin(repo_config.url.as_str())?;

    // Repository content is pulled
    git::fetch()?;
    git::pull(false)?;

    // Git LFS is initialized
    git::lfs_track("*.dat")?;
    git::lfs_track("*.mca")?;

    // .gitignore is created if non existing yet
    let gitignore_path = git::REPO_PATH.to_owned() + "/.gitignore";
    let gitignore_path = gitignore_path.as_str();

    if !file::exists(gitignore_path) {
        file::write(gitignore_path, 
            "./libraries\n./logs\n./versions")?;
    }

    // .host file is created if not existing in the remote repo
    let hostfile_path = git::REPO_PATH.to_owned() + HOSTFILE_PATH;
    let hostfile_path = hostfile_path.as_str();

    if !file::exists(hostfile_path) {
        file::touch(hostfile_path)?;
    }

    Ok(())
}

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