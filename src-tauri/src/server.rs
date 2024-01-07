pub mod servercfg;

use std::process::Child;
use std::io::Write;

use servercfg::ServerConfig;
use crate::ioutils::terminal;




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
    pub fn get_configuration(&self) -> &Option<ServerConfig> {
        &self.config
    }

    pub fn set_configuration(&mut self, config: ServerConfig) {
        self.config = Some(config);
    }

    pub fn load_configuration(&mut self) {
        match ServerConfig::load_config() {
            Ok(cfg) => self.config = Some(cfg),
            Err(_) => self.config = None,
        }
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

    pub fn run(&mut self) -> Result<(), ServerError> {
        if let None = self.config {
            return Err(ServerError::NO_CONFIG)
        }
        match &(self.state) {
            State::RUNNING(pid) => Err(ServerError::RUNNING(*pid)),
            State::HOSTED(host) => Err(ServerError::HOSTED(String::from(host))),
            State::STOPPED => { 
                let pid = self.execute_server_jar()?;
                self.state = State::RUNNING(pid);
                Ok(())
            }
        }
    }

    pub fn stop(&mut self) -> Result<(), ServerError> {
        match &mut self.process {
            Some(ref mut child) => {
                let child_stdin = child.stdin.take();
                let _ = child_stdin.unwrap().write_all("stop".as_bytes());
                let _ = child.wait();

                self.state = State::STOPPED;

                Ok(())
            },
            None => {
                Err(ServerError::NOT_FOUND)
            }
        }
    }
        

    // =========== PRIVATE FUNCTIONS ===========
    fn execute_server_jar(&mut self) -> Result<u32, ServerError> {
        if let Some(config) = &(self.config) {
            let program = "java";
            let dir = "mojang/";
            let mut args = [
                format!("-Xmx{}", config.get_mem_max()),
                format!("-Xms{}", config.get_mem_init()),
                String::from("-jar"),
                String::from("server.jar"),
                String::from("")
            ];

            if !config.get_gui() { args[4] = String::from("--nogui"); }

            let command_result = terminal::spawn_process(
                program, args, dir, LOG_PATH
            );

            match command_result {
                Ok(child) => {
                    let pid = child.id();
                    self.process = Some(child);
                    Ok(pid)
                }
                Err(_) => Err(ServerError::JAR_FAIL)
            } 
        }
        else { Err(ServerError::NO_CONFIG) }
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