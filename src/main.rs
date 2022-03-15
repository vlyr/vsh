use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};

fn main() {
    let mut stdout = std::io::stdout();
    enable_raw_mode().unwrap();

    let mut cmdbuf = String::new();
    let prompt = "vsh | ";

    'cmdloop: loop {
        execute!(stdout, cursor::MoveToColumn(1), Print(prompt)).unwrap();
        cmdbuf.clear();

        'keyloop: loop {
            let event = read().unwrap();

            match event {
                Event::Key(k) => match k.code {
                    KeyCode::Enter => {
                        execute!(stdout, cursor::MoveToColumn(1), Print("\n")).unwrap();

                        std::process::Command::new(&cmdbuf)
                            .spawn()
                            .unwrap()
                            .wait()
                            .unwrap();
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
                },
                _ => (),
            }

            execute!(
                stdout,
                cursor::MoveToColumn(1),
                Clear(ClearType::CurrentLine),
                Print(prompt),
                Print(&cmdbuf),
            )
            .unwrap();
        }
    }

    disable_raw_mode().unwrap();
}
