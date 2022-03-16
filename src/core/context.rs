use std::env;
use std::error::Error;
use std::path::PathBuf;

pub struct Context {
    current_dir: PathBuf, // or Path?
}

impl Context {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let home_dir = env::current_dir()?;

        Ok(Self {
            current_dir: home_dir,
        })
    }

    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn current_dir_str(&self) -> String {
        let dir_str = self.current_dir().display().to_string();
        let home_dir = env::var("HOME").expect("Failed getting environment variable $HOME");
        dir_str.replace(&home_dir, "~")
    }
}
