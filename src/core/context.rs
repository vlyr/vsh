use crate::core::completion::*;
use crossterm::cursor;
use crossterm::execute;
use crossterm::style::Print;

use std::env;
use std::error::Error;
use std::io::Stdout;
use std::path::{Path, PathBuf};

pub struct Context {
    current_dir: PathBuf,
    command_buffer: String,
    completion_state: CompletionState,
}

impl Context {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let home_dir = env::current_dir()?;

        Ok(Self {
            command_buffer: String::new(),
            current_dir: home_dir,
            completion_state: CompletionState::Inactive,
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

    pub fn completion_state(&self) -> &CompletionState {
        &self.completion_state
    }

    pub fn completion_state_mut(&mut self) -> &mut CompletionState {
        &mut self.completion_state
    }

    pub fn print_completions(&self, stdout: &mut Stdout) {
        execute!(
            stdout,
            cursor::SavePosition,
            Print("\n"),
            cursor::MoveToColumn(1)
        )
        .unwrap();

        if let CompletionState::Active(ref state) = self.completion_state {
            for completion in state.completions() {
                execute!(stdout, Print(completion), Print(" ")).unwrap();
            }
        }

        execute!(stdout, cursor::RestorePosition, cursor::MoveUp(1)).unwrap();
    }

    pub fn current_dir_str(&self) -> String {
        let dir_str = self.current_dir().display().to_string();
        let home_dir = env::var("HOME").expect("Failed getting environment variable $HOME");
        dir_str.replace(&home_dir, "~")
    }
}
