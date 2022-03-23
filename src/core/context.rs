use crossterm::cursor;
use crossterm::execute;
use crossterm::style::Print;

use std::env;
use std::error::Error;
use std::fs;
use std::io::Stdout;
use std::mem;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ActiveCompletionState {
    completions: Vec<String>,
    idx: usize,
}

impl Default for ActiveCompletionState {
    fn default() -> Self {
        Self {
            completions: vec![],
            idx: 0,
        }
    }
}
impl ActiveCompletionState {
    pub fn new(completions: Vec<String>) -> Self {
        Self {
            completions,
            idx: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompletionState {
    Active(ActiveCompletionState),
    Inactive,
}

impl CompletionState {
    pub fn active() -> Self {
        let state = ActiveCompletionState::default();

        Self::Active(state)
    }

    pub fn toggle(&mut self) {
        let mut new = match self {
            CompletionState::Inactive => {
                let inner = ActiveCompletionState::default();
                CompletionState::Active(inner)
            }

            CompletionState::Active(_) => CompletionState::Inactive,
        };

        mem::swap(self, &mut new);
    }

    pub fn next_completion(&mut self) {
        if let Self::Active(ref mut state) = self {
            state.idx += 1;
        }
    }

    pub fn prev_completion(&mut self) {
        if let Self::Active(ref mut state) = self {
            if state.idx > 0 {
                state.idx -= 1;
            }
        }
    }

    pub fn dir_completions(&mut self) {
        let path = std::env::current_dir().unwrap();

        let mut file_names = vec![];

        for file in fs::read_dir(path).unwrap().filter_map(|entry| entry.ok()) {
            let name = file.file_name();
            file_names.push(name.clone());
        }

        let mut file_names: Vec<String> = file_names
            .iter()
            .map(|x| x.to_str().unwrap().to_string())
            .collect();

        file_names.sort();

        let inner_state = ActiveCompletionState::new(file_names);
        mem::swap(self, &mut CompletionState::Active(inner_state));
    }
}

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
        execute!(stdout, Print("\n"), cursor::MoveToColumn(1)).unwrap();

        if let CompletionState::Active(ref state) = self.completion_state {
            for completion in &state.completions {
                execute!(stdout, Print(completion), Print(" ")).unwrap();
            }
        }

        execute!(stdout, Print("\n")).unwrap();
    }

    pub fn current_dir_str(&self) -> String {
        let dir_str = self.current_dir().display().to_string();
        let home_dir = env::var("HOME").expect("Failed getting environment variable $HOME");
        dir_str.replace(&home_dir, "~")
    }
}
