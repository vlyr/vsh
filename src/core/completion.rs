use crossterm::{cursor, execute, style::Print};
use std::error::Error;
use std::fs;
use std::io::Stdout;

pub enum CompletionType {
    // Check directory from input string
    Files,
    // Use PATH variable
    Binaries,
}
