use crate::core::{input::LoopControl, utils::error_handler, Context};
use std::error::Error;
use std::process::Command;

pub fn execute<'a, 'b>(
    cmd: &'a str,
    mut args: impl Iterator<Item = &'b str>,
    context: &mut Context,
) -> Result<LoopControl, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(LoopControl::NextCommand);
    }

    match cmd {
        "cd" => match args.next() {
            Some(p) => {
                context.set_current_dir(p.to_string());
                Ok(LoopControl::NextCommand)
            }
            None => {
                let mut stdout = std::io::stdout();
                error_handler(
                    &mut stdout,
                    &format!("Failed executing command: Invalid arguments provided"),
                );

                Ok(LoopControl::NextCommand)
            }
        },
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
