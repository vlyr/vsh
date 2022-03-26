use crossterm::{
    cursor,
    event::{read, Event},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use std::env;
use std::error::Error;
use std::io;
use vsh::core::{
    completion::CompletionState,
    input::{handle_key, LoopControl},
    utils::{error_handler, format_path},
    Context,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    let mut context = Context::new()?;

    enable_raw_mode().unwrap();

    'cmdloop: loop {
        let current_dir = format_path(env::current_dir()?.to_str().unwrap());
        let prompt = format!("{} | ", current_dir);

        if let CompletionState::Inactive = context.completion_state() {
            execute!(stdout, cursor::MoveToColumn(1), Print(&prompt)).unwrap();
            context.command_buffer_mut().clear();
        }

        loop {
            let event = read().unwrap();

            match event {
                Event::Key(key_event) => match handle_key(&mut stdout, &mut context, key_event)? {
                    LoopControl::NextCommand => continue 'cmdloop,
                    LoopControl::NextKey => (),
                    LoopControl::Exit => break 'cmdloop,
                },
                _ => (),
            }

            execute!(
                stdout,
                cursor::MoveToColumn(1),
                Clear(ClearType::CurrentLine),
                Print(&prompt),
                Print(context.command_buffer()),
            )
            .unwrap();
        }
    }

    if let Err(e) = disable_raw_mode() {
        error_handler(
            &mut stdout,
            &format!("Failed disabling raw mode on shell exit: {}", e),
        );
    }

    Ok(())
}
