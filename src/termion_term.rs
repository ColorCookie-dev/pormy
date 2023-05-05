use crate::terminal::{TimerCommand, TimerScreen, CommandRecvError};
use std::io::{ stdout, Write, StdoutLock, stdin };
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use termion::{
    raw::RawTerminal,
    raw::IntoRawMode,
    input::TermRead,
    clear,
    cursor::Goto,
    event::Key,
};

pub struct TermionScreen<'a> {
    screen: RawTerminal<StdoutLock<'a>>,
    cmd_reciever: Receiver<Result<Key, std::io::Error>>,
}


impl<'a> TimerScreen for TermionScreen<'a> {
    fn init() -> Result<Self, std::io::Error> {
        let (tx, rx) = channel();
        let _join_handle = std::thread::spawn(move || {
            loop {
                let stdin = stdin().lock();
                for key in stdin.keys() {
                    tx.send(key).unwrap();
                }
            }
        });

        Ok(TermionScreen {
            screen: stdout().lock().into_raw_mode()?,
            cmd_reciever: rx,
        })
    }

    fn build_timer_screen(&mut self, time_elapsed: Duration)
        -> Result<(), std::io::Error> {
        write!(self.screen, "{}", clear::All)?;
        write!(self.screen, "{}{}", Goto(1, 1), time_elapsed.as_secs())?;
        write!(self.screen, "{}{}", Goto(1, 2), "[Space]: Start/Stop timer")?;
        write!(self.screen, "{}{}", Goto(1, 3), termion::cursor::Hide)?;
        Ok(())
    }

    fn pop_cmd(&self) -> Result<TimerCommand, CommandRecvError> {
        TermionScreen::<'a>::handle_key(self.cmd_reciever.try_recv()??)
            .ok_or(CommandRecvError::NotACommand)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.screen.flush()
    }
}

impl<'a> Drop for TermionScreen<'a> {
    fn drop(&mut self) {
        write!(self.screen, "{}", termion::cursor::Show).unwrap();
        self.flush().unwrap();
    }
}

impl<'a> TermionScreen<'a> {
    fn handle_key(key: Key) -> Option<TimerCommand> {
        match key {
            Key::Char('q') => Some(TimerCommand::Quit),
            Key::Char(' ') => Some(TimerCommand::Toggle),
            _ => None,
        }
    }
}
