use crossterm::{cursor, execute, style::Print};
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::Stdout;

pub fn error_handler<T: fmt::Display>(stdout: &mut Stdout, msg: &T) {
    execute!(stdout, Print(msg), Print("\n"), cursor::MoveToColumn(1)).unwrap();
}

pub fn format_path<P: AsRef<str>>(path: P) -> String {
    let home_dir = env::var("HOME").expect("Failed getting environment variable $HOME");
    path.as_ref().replace(&home_dir, "~")
}

pub fn append_to_path<S: AsRef<str>>(dest: S, src: S) -> String {
    let mut dest = dest.as_ref().to_string();
    dest.push_str(src.as_ref());

    let mut components = vec![];

    for (idx, component) in dest.split('/').enumerate() {
        match component {
            ".." => {
                if idx > 0 {
                    components.remove(idx - 1);
                }
            }
            "." => (),

            _ => components.push(component),
        }
    }

    components.join("/")
}

pub fn get_path_binaries() -> Result<Vec<String>, Box<dyn Error>> {
    Ok(env::var("PATH")?
        .split(":")
        .map(|path| match fs::read_dir(path) {
            Ok(dir_entries) => dir_entries
                .map(|entry| match entry {
                    Ok(file) => file.file_name().to_str().unwrap().to_string(),
                    Err(_) => String::new(),
                })
                .collect(),

            Err(_) => vec![],
        })
        .flatten()
        .filter(|name| !name.is_empty())
        .collect())
}
