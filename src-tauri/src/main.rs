// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hostcraft::repo;
use hostcraft::server as srvr;
use hostcraft::server::Server;
use hostcraft::server::servercfg;
use hostcraft::ioutils::file;




// =============================================================
// ===================== SERVER COMMANDS =======================
// =============================================================

static mut SERVER: Option<Server> = None;

#[tauri::command]
fn run_server() -> Result<(), String> {
  unsafe {
    if let Some(ref mut server) = SERVER {
      match server.run() {
        Ok(_) => { Ok(()) },
        Err(err) => { Err(srvr::get_error_msg(err)) }
      }
    }
    else {
      Err(srvr::get_error_msg(srvr::ServerError::NOT_FOUND))
    }
  }
}

#[tauri::command]
fn stop_server() -> Result<(), String> {
  unsafe {
    if let Some(ref mut server) = SERVER {
      match server.stop() {
        Ok(_) => { Ok(()) },
        Err(err) => { Err(srvr::get_error_msg(err)) }
      }
    }
    else {
      Err(srvr::get_error_msg(srvr::ServerError::NOT_FOUND))
    }
  }
}

#[tauri::command]
fn get_server_config() -> Result<String, String> {
  let server_config = file::read(servercfg::get_config_filepath().as_str());

  match server_config {
    Ok(content) => { Ok(content) },
    Err(_) => { Err(String::from("Fail reading server configuration file!")) }
  }
}

#[tauri::command(rename_all = "snake_case")]
fn update_server_config(json_config: &str) -> Result<(), String> {
  let write_result = file::write(servercfg::get_config_filepath().as_str(), json_config);

  match write_result {
    Ok(_) => { 
      unsafe {
        if let Some(ref mut server) = SERVER {
          server.load_configuration();
          Ok(())
        } 
        else {
          Err(srvr::get_error_msg(srvr::ServerError::NOT_FOUND))
        }
      }
    },
    Err(_) => { Err(String::from("Fail updating server configuration file!")) }
  }
}




// =============================================================
// ====================== REPO COMMANDS ========================
// =============================================================
#[tauri::command]
fn is_repo_initialized() -> bool {
  repo::is_repo_initialized()
}

#[tauri::command(rename_all = "snake_case")]
fn init_repo(repo_config: &str) -> Result<(), String> {
  let init_result = repo::init_repo(repo_config);

  match init_result {
    Ok(_) => { Ok(()) },
    Err(_) => { Err(String::from("Could not initialize repository locally!")) }
  }
}

#[tauri::command]
fn get_repo_config() -> Result<String, String> {
  let repo_config = repo::get_repo_json_config();

  match repo_config {
    Ok(json_config) => { Ok(json_config) },
    Err(_) => { Err(String::from("Could not read the local Git repo configuration!")) }
  }
}

#[tauri::command(rename_all = "snake_case")]
fn update_repo_config(json_config: &str) -> Result<(), String> {
  let update_result = repo::set_repo_json_config(json_config);

  match update_result {
    Ok(_) => { Ok(()) },
    Err(_) => { Err(String::from("Could not update the local Git repo configuration!")) }

  }
}




fn main() {
  unsafe {
    SERVER = Some(Server::new());
  }

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_server_config, update_server_config,
      run_server, stop_server,
      is_repo_initialized, init_repo,
      get_repo_config, update_repo_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
