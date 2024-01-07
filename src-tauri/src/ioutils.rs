pub mod terminal {
    use std::process::{Command, Child, Stdio, Output};
    use std::ffi::OsStr;

    pub fn spawn_process<I, S>(
        program: &str,
        args: I,
        dir: &str,
        stdout_filepath: &str,
    ) -> Result<Child, std::io::Error> 
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    { 
        let mut process: Command = Command::new(program);
        process
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::from(
            super::file::open_file(stdout_filepath)
            .expect("Failed opening stdout file")
        )).args(args);

        process.spawn()
    }

    pub fn execute_command<I, S>(
        program: &str,
        args: I,
        dir: &str
    ) -> Result<Output, std::io::Error>
    where 
        I: IntoIterator<Item = S>,
        S: AsRef <OsStr>,
    {
        let mut command: Command = Command::new(program);
        command
        .current_dir(dir)
        .args(args)
        .output()
    }
}

pub mod file {
    use std::fs::{File, OpenOptions};
    use std::io::{Write, Read};
    use std::path::Path;

    pub fn open_file(filepath: &str) -> Result<File, std::io::Error> {
        OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filepath)
    }

    pub fn open_read_file(filepath: &str) -> Result<File, std::io::Error> {
        OpenOptions::new()
        .read(true)
        .open(filepath)
    }

    pub fn write(filepath: &str, content: &str) -> Result<(), std::io::Error> {
        let mut file = open_file(filepath)?;
        file.write_all(content.as_bytes())
    }

    pub fn touch(filepath: &str) -> Result<(), std::io::Error> {
        write(filepath, "")
    }

    pub fn read(filepath: &str) -> Result<String, std::io::Error> {
        let mut file = open_read_file(filepath)?;
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(buffer)
    }

    pub fn exists(filepath: &str) -> bool {
        Path::new(filepath).exists()
    }
}