use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

use std::error::Error;
use std::fmt;
use std::io::{self, Stdout};
use vsh::core::Context;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    let ctx = Context::new()?;

    enable_raw_mode().unwrap();

    let mut cmdbuf = String::new();

    'cmdloop: loop {
        let prompt = format!("{} | ", ctx.current_dir_str());
        execute!(stdout, cursor::MoveToColumn(1), Print(&prompt)).unwrap();
        cmdbuf.clear();

        'keyloop: loop {
            let event = read().unwrap();

            match event {
                Event::Key(k) => {
                    match k.code {
                        KeyCode::Enter => {
                            execute!(stdout, cursor::MoveToColumn(1), Print("\n")).unwrap();
                            let mut args = cmdbuf.trim().split_whitespace();

                            let cmd = args.next();

                            match cmd {
                                Some(cmd) => {
                                    if cmd == "exit" {
                                        break 'cmdloop;
                                    }

                                    // Disabling raw mode is required in order for commands to function
                                    // normally
                                    if let Err(e) = disable_raw_mode() {
                                        error_handler(
                                            &mut stdout,
                                            &format!("Failed enabling raw mode after command execution: {}", e),
                                        );
                                        continue 'cmdloop;
                                    }

                                    std::process::Command::new(&cmd)
                                        .args(&mut args)
                                        .spawn()
                                        .unwrap()
                                        .wait()
                                        .unwrap();

                                    if let Err(e) = enable_raw_mode() {
                                        error_handler(
                                            &mut stdout,
                                            &format!("Failed enabling raw mode after command execution: {}", e),
                                        );
                                    }
                                }

                                None => continue 'cmdloop,
                            }

                            break 'keyloop;
                        }

                        KeyCode::Backspace => {
                            cmdbuf.pop();
                            stdout.execute(cursor::MoveLeft(1)).unwrap();
                        }

                        KeyCode::Char(c) => cmdbuf.push(c),

                        KeyCode::Esc => {
                            break 'cmdloop;
                        }

                        _ => (),
                    }
                }
                _ => (),
            }

            execute!(
                stdout,
                cursor::MoveToColumn(1),
                Clear(ClearType::CurrentLine),
                Print(&prompt),
                Print(&cmdbuf),
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

fn error_handler<T: fmt::Display>(stdout: &mut Stdout, msg: &T) {
    execute!(
        stdout,
        Print("\n"),
        Print(msg),
        Print("\n"),
        cursor::MoveToColumn(1)
    )
    .unwrap();
}
