use std::error::Error;
use std::str;
use serde::{Serialize, Deserialize};

use crate::ioutils;

const CONFIG_FILEPATH: &str = "conf/server.conf";

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    mem_max: String,
    mem_init: String,
    gui: bool,
}

// TODO: store configuration in a config file
// Maybe separate Server and ServerConfig in different
// files / modules?
impl ServerConfig {
    pub fn new(
        mem_max: String, mem_init: String, gui: bool
    ) ->  Result<ServerConfig, Box<dyn Error>>{
        let server_config = ServerConfig {
            mem_max: mem_max,
            mem_init: mem_init,
            gui: gui,
        };

        ioutils::file::write(CONFIG_FILEPATH, server_config.to_json()?.as_str())?;

        Ok(server_config)
    }

    pub fn load_config() -> Result<ServerConfig, Box<dyn Error>>{
        let json_config = ioutils::file::read(CONFIG_FILEPATH)?;
        let server_config: ServerConfig = serde_json::from_str(json_config.as_str().trim())?;

        Ok(server_config)
    }

    pub fn get_mem_max(&self) -> &str {
        self.mem_max.as_str()
    }

    pub fn get_mem_init(&self) -> &str {
        self.mem_init.as_str()
    }

    pub fn get_gui(&self) -> &bool {
        &(self.gui)
    }

    pub fn to_string(&self) -> String {
       format!(
        "Max memory = {}\n\
        Initial memory = {}\n\
        GUI = {}\n\
        ", self.mem_max, self.mem_init, self.gui
        )
    }

    fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let self_json = serde_json::to_string(&self)?;
        
        Ok(self_json)
    } 
}

pub fn get_config_filepath() -> String {
    String::from(CONFIG_FILEPATH)
}