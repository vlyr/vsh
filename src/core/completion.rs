use std::error::Error;
use std::io::Stdout;
use std::path::Path;

pub enum CompletionType {
    // Check directory from input string
    Files,
    // Use PATH variable
    Binaries,
}

pub fn show_completions<I: AsRef<str>>(
    stdout: &mut Stdout,
    input_string: &I,
    completion_type: CompletionType,
) -> Result<(), Box<dyn Error>> {
    match completion_type {
        CompletionType::Files => Ok(()),
        _ => Ok(()),
    }
}
