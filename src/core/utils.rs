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
