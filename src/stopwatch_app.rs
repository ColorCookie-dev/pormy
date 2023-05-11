use crate::prelude::*;

use crate::stopwatch;
use crate::terminal;

use crate::terminal::TimerCommand;
use std::time::Duration;

pub struct StopwatchApp<TimerScreen: terminal::TimerScreen> {
    stopwatch: stopwatch::Stopwatch,
    terminal: TimerScreen,
    quit: bool,
}

impl<TS: terminal::TimerScreen> StopwatchApp<TS> {
    pub fn new() -> Result<Self> {
        // Setup raw mode for terminal
        let terminal = TS::init()?;

        // Stopwatch instance
        let stopwatch = stopwatch::Stopwatch::start();

        Ok(Self {
            stopwatch,
            terminal,
            quit: false,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.terminal.build_timer_screen(self.stopwatch.time_elapsed())?;
        self.terminal.flush()?;

        self.terminal.pop_cmd()?.map(|cmd| self.handle_command(cmd));

        std::thread::sleep(Duration::from_millis(60));
        Ok(())
    }

    pub fn handle_command(&mut self, cmd: TimerCommand) {
        match cmd {
            TimerCommand::Toggle => {
                self.stopwatch.toggle();
            },
            TimerCommand::Quit => {
                self.stopwatch.pause();
                self.quit = true;
            }
        }
    }

    pub fn to_quit(&self) -> bool {
        self.quit
    }
}

