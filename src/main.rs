mod format;
use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Mutex};
use termion::{clear, color, cursor, event::Key, input::TermRead, raw::IntoRawMode};

macro_rules! raw_writeln {
    ($out: expr) => {
        writeln!($out)
    };

    ($out: expr, $($arg: tt)*) => {{
        let s = format!($($arg)*);
        writeln!($out, "{line}{back}", line = s, back = cursor::Left(s.len() as u16))
    }};
}

macro_rules! raw_println {
    ($out: expr) => {
        println!();
        stdout().flush().unwrap();
    };

    ($($arg: tt)*) => {
        let s = format!($($arg)*);
        println!("{line}{back}", line = s, back = cursor::Left(s.len() as u16));
        stdout().flush().unwrap();
    };
}

enum Status {
    Running,
    Paused,
    Quit,
}

fn main() -> anyhow::Result<()> {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

    let _stdout = stdout().into_raw_mode()?;
    let status = Arc::new(Mutex::new(Status::Running));
    let status_new = Arc::clone(&status);
    let join_handle = input_listener(move |cmd| match cmd {
        Command::Toggle => {
            let mut status = status_new.lock().unwrap();
            *status = match *status {
                Status::Running => Status::Paused,
                Status::Paused => Status::Running,
                Status::Quit => Status::Quit,
            };
        }

        Command::Quit => {
            let mut status = status_new.lock().unwrap();
            *status = Status::Quit;
        }
    });

    for i in 1.. {
        {
            let status = status.lock().unwrap();
            match *status {
                Status::Quit => break,
                Status::Running => {
                    clear_screen()?;
                    raw_println!("{}", format::fmt_time(i));
                    raw_println!("{}", "[Space]: Start/Stop timer");
                }
                Status::Paused => (),
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(250));
    }

    join_handle.join().unwrap();

    Ok(())
}

enum Command {
    Toggle,
    Quit,
}

fn input_listener(callback: impl Fn(Command) + Send + 'static) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let stdin = stdin().keys();
        for key in stdin {
            match key.unwrap() {
                Key::Char('q') => {
                    callback(Command::Quit);
                    break;
                }
                Key::Char(' ') => callback(Command::Toggle),
                _ => (),
            }
        }
    })
}

fn clear_screen() -> Result<(), std::io::Error> {
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!(
        "{clear}{goto}",
        clear = clear::All,
        goto = cursor::Goto(1, 1)
    );
    std::io::stdout().flush()
}
