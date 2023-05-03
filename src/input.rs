use std::thread;
use std::{io::stdin, sync::mpsc};
use termion::{event::Key, input::TermRead};

pub enum Command {
    Quit,
    Space,
    Enter,
}

pub fn listen_command() -> mpsc::Receiver<Command> {
    let (tx, rx) = mpsc::sync_channel::<Command>(3);

    thread::spawn(move || {
        let stdin = stdin().keys();

        for c in stdin {
            match c {
                Ok(Key::Char('q')) => tx.try_send(Command::Quit).ok(),
                Ok(Key::Char(' ')) => tx.try_send(Command::Space).ok(),
                Ok(Key::Char('\n')) => tx.try_send(Command::Enter).ok(),
                _ => None,
            };
        }
    });

    rx
}
