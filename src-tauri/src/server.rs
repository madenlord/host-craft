pub mod servercfg;

use std::process::Child;

use servercfg::ServerConfig;




//=================================================================
//======================   CONSTANTS & OOP   ======================
//=================================================================
const LOG_PATH: &str = "logs/server_jar.log";

pub struct Server {
    config: Option<ServerConfig>,
    state: State,
    process: Option<Child>,
}

enum State {
    HOSTED(String),
    RUNNING(u32),
    STOPPED,
}

pub enum ServerError {
    NO_CONFIG,
    RUNNING(u32),
    HOSTED(String),
    JAR_FAIL,
    NOT_FOUND,
    IO_ERROR(String),
    REPO_FAIL
}

impl Server {
    pub fn new() -> Server {
        let server_config: Option<ServerConfig>;

        match ServerConfig::load_config() {
            Ok(cfg) => server_config = Some(cfg),
            Err(_) => server_config = None,
        };

        Server { 
            config: server_config,
            state: State::STOPPED,
            process: None, 
        }
    }

    // =========== SETTERS AND GETTERS ===========
    pub fn get_config(&self) -> &Option<ServerConfig> {
        &self.config
    }

    pub fn set_configuration(&mut self, config: ServerConfig) {
        self.config = Some(config);
    }

    pub fn get_state(&self) -> String {
        match &self.state {
            State::HOSTED(host) => format!("Hosted by {}\n", host),
            State::RUNNING(pid) => format!("Server running (PID = {})\n", pid),
            State::STOPPED => String::from("Server is not running.\n"), 
        }
    }

    // =========== FUNCTIONALITY ===========
    pub fn is_configured(&self) -> bool {
        if let None = self.config { false }
        else { true }
    }
}




//=================================================================
//====================   AUXILIAR FUNCTIONS   =====================
//=================================================================
pub fn get_error_msg(err: ServerError) -> String {
    match err {
        ServerError::NO_CONFIG => String::from("Server has not been configured yet!\n"),
        ServerError::RUNNING(pid) => format!("Server is already running! (PID = {})\n", pid),
        ServerError::HOSTED(host) => format!("Server is being hosted by {}\n", host.as_str()),
        ServerError::JAR_FAIL => String::from("mojang/server.jar execution failed!\n"),
        ServerError::NOT_FOUND => String::from("Server instance could not be found!\n"),
        ServerError::IO_ERROR(file) => format!("I/O ERROR: could not read '{}'!\n", file),
        ServerError::REPO_FAIL => String::from("Could not reach the world's data repository!\n"),
    }
}