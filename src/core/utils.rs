use crossterm::{cursor, execute, style::Print};
use std::env;
use std::fmt;
use std::io::Stdout;

pub fn error_handler<T: fmt::Display>(stdout: &mut Stdout, msg: &T) {
    execute!(stdout, Print(msg), Print("\n"), cursor::MoveToColumn(1)).unwrap();
}

pub fn format_path<P: AsRef<str>>(path: P) -> String {
    let home_dir = env::var("HOME").expect("Failed getting environment variable $HOME");
    path.as_ref().replace(&home_dir, "~")
}
