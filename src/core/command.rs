use crate::core::{input::LoopControl, utils::error_handler};
use std::error::Error;
use std::process::Command;

pub fn execute<'a, 'b>(
    cmd: &'a str,
    mut args: impl Iterator<Item = &'b str>,
) -> Result<LoopControl, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(LoopControl::NextCommand);
    }

    match cmd {
        "cd" => unimplemented!(),
        "exit" => Ok(LoopControl::Exit),
        _ => {
            match Command::new(&cmd).args(&mut args).spawn() {
                Ok(mut handle) => {
                    handle.wait()?;
                }
                Err(e) => {
                    let mut stdout = std::io::stdout();
                    error_handler(&mut stdout, &format!("Failed executing command: {}", e));
                }
            }
            Ok(LoopControl::NextCommand)
        }
    }
}
