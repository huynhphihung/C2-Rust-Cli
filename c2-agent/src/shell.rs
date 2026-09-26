use std::{
    io::Result,
    path::PathBuf,
    process::{Command, Output},
};

pub struct Shell {
    pub program: String,
    pub current_dir: PathBuf,
}

impl Shell {
    pub fn new() -> Self {
        let program = if cfg!(target_os = "windows") {
            "powershell.exe".to_string()
        } else {
            "bash".to_string()
        };

        Self {
            program,
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn execute(&self, command: &str) -> Result<Output> {
        if cfg!(target_os = "windows") {
            Command::new(&self.program)
                .arg("-Command")
                .arg(command)
                .current_dir(&self.current_dir)
                .output()
        } else {
            Command::new(&self.program)
                .arg("-c")
                .arg(command)
                .current_dir(&self.current_dir)
                .output()
        }
    }
}
