use crossterm::{cursor, execute, style::Print};
use std::fs;
use std::mem;

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

    pub fn completions(&self) -> &Vec<String> {
        &self.completions
    }

    pub fn next_completion(&mut self) {
        self.idx += 1;
    }

    pub fn prev_completion(&mut self) {
        if self.idx > 0 {
            self.idx -= 1;
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

    pub fn reset(&mut self) {
        mem::swap(self, &mut CompletionState::Inactive);
    }

    pub fn dir_completions(&mut self, command_buffer: &String) {
        let args: Vec<&str> = command_buffer.split(" ").collect();
        let last_arg = args.get(args.len() - 1).unwrap_or(&"").trim();

        let path = std::env::current_dir().unwrap();

        let mut file_names = vec![];

        for file in fs::read_dir(path)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|file| file.file_name().to_str().unwrap().starts_with(last_arg))
        {
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
