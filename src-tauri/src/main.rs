// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_server_config
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
