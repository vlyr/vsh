use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

pub struct Context {
    current_dir: PathBuf,
    command_buffer: String,
}

impl Context {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let home_dir = env::current_dir()?;

        Ok(Self {
            command_buffer: String::new(),
            current_dir: home_dir,
        })
    }

    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn set_current_dir(&mut self, dir: String) {
        let path = Path::new(&dir).to_path_buf();

        if path.exists() && path.is_dir() {
            self.current_dir = path;
            std::env::set_current_dir(dir).unwrap();
        }
    }

    pub fn command_buffer(&self) -> &String {
        &self.command_buffer
    }

    pub fn command_buffer_mut(&mut self) -> &mut String {
        &mut self.command_buffer
    }

    pub fn current_dir_str(&self) -> String {
        let dir_str = self.current_dir().display().to_string();
        let home_dir = env::var("HOME").expect("Failed getting environment variable $HOME");
        dir_str.replace(&home_dir, "~")
    }
}
