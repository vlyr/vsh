use crate::core::utils::error_handler;
use crate::core::{command::execute as execute_command, Context};
use crossterm::event::KeyEvent;
use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::error::Error;
use std::io::Stdout;

pub mod key;
pub use key::Key;

pub enum LoopControl {
    NextCommand,
    NextKey,
    Exit,
}

pub fn handle_key(
    stdout: &mut Stdout,
    context: &mut Context,
    event: KeyEvent,
) -> Result<LoopControl, Box<dyn Error>> {
    let key = Key::from(event);
    use LoopControl::*;

    match key {
        Key::Char(c) => {
            context.command_buffer_mut().push(c);
            Ok(NextKey)
        }

        Key::Escape => Ok(Exit),

        Key::Backspace => {
            context.command_buffer_mut().pop();
            stdout.execute(cursor::MoveLeft(1))?;
            Ok(NextKey)
        }

        Key::Enter => {
            execute!(stdout, cursor::MoveToColumn(1), Print("\n"))?;
            let mut args = context.command_buffer().trim().split_whitespace();

            let cmd = args.next();

            match cmd {
                Some(cmd) => {
                    // Disabling raw mode is required in order for commands to function
                    // normally
                    if let Err(e) = disable_raw_mode() {
                        error_handler(
                            stdout,
                            &format!("Failed enabling raw mode after command execution: {}", e),
                        );

                        return Ok(NextCommand);
                    }

                    let ret_status = execute_command(cmd, args)?;

                    if let Err(e) = enable_raw_mode() {
                        error_handler(
                            stdout,
                            &format!("Failed enabling raw mode after command execution: {}", e),
                        );
                    }

                    Ok(ret_status)
                }

                None => Ok(NextCommand),
            }
        }
        _ => Ok(NextKey),
    }
}
