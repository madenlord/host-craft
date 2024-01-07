mod git;

use std::error::Error as DynError;
use std::str;
use std::fmt;

use serde::{Serialize, Deserialize};

use super::ioutils::file;

const HOSTFILE_REL_PATH: &str = "/.host";


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

// =========== PERFORM ACTIONS ===========
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
    let hostfile_path = get_hostfile_path();
    let hostfile_path = hostfile_path.as_str();

    if !file::exists(hostfile_path) {
        file::touch(hostfile_path)?;
    }

    Ok(())
}

pub fn download_world_data_updates() -> Result<bool, std::io::Error> {
    let mut update_found: bool = false;

    // Get latest changes in repo
    let update_res = git::fetch()?;

    // If there are updates to be downloaded:
    if !update_res.stdout.is_empty() || !update_res.stderr.is_empty() {
        update_found = true;
        git::pull(false)?;
    }

    Ok(update_found)
}

pub fn upload_world_data() -> Result<(), std::io::Error> {
    git::add(vec!["*"])?;
    git::commit("World data update!")?;
    git::push()?;

    Ok(())
}

pub fn commit_host(user: &str) -> Result<(), std::io::Error> {
    // Current hostname is written in the hostfile and
    // changes are pushed to the Git repo
    update_hostfile(user)?;
    git::add(vec![HOSTFILE_REL_PATH])?;
    git::commit(format!("{} is now hosting the server!", user).as_str())?;

    Ok(())
}

pub fn update_hostfile(user: &str) -> Result<(), std::io::Error> {
    file::write(get_hostfile_path().as_str(), user)?;

    Ok(())
}

// =========== GETTERS & SETTERS ===========
pub fn get_repo_host() -> Result<String, std::io::Error> {
    let hostfile_content = file::read(get_hostfile_path().as_str())?;
    Ok(String::from(hostfile_content.trim()))
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

pub fn get_hostfile_path() -> String {
    String::from(git::REPO_PATH.to_owned() + HOSTFILE_REL_PATH)
}

pub fn who_am_i() -> Result<String, Box<dyn DynError>> {
    let repo_config = get_repo_config()?;
    Ok(repo_config.username) 
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