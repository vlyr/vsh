use crossterm::{cursor, execute, style::Print};
use std::fmt;
use std::io::Stdout;

pub fn error_handler<T: fmt::Display>(stdout: &mut Stdout, msg: &T) {
    execute!(stdout, Print(msg), Print("\n"), cursor::MoveToColumn(1)).unwrap();
}
