// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hostcraft::repo;
use hostcraft::ioutils::file;
use hostcraft::server::servercfg;

#[tauri::command(rename_all = "snake_case")]
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
    Ok(_) => { Ok(()) },
    Err(_) => { Err(String::from("Fail updating server configuration file!")) }
  }
}

#[tauri::command(rename_all = "snake_case")]
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
    Ok(ok) => { Ok(()) },
    Err(_) => { Err(String::from("Could not update the local Git repo configuration!")) }

  }
}




fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_server_config, update_server_config,
      get_repo_config, update_repo_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
