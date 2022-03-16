use crate::core::input::LoopControl;
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
            Command::new(&cmd).args(&mut args).spawn()?.wait()?;
            Ok(LoopControl::NextCommand)
        }
    }
}
