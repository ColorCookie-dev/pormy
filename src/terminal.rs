use crate::prelude::*;
use std::time::Duration;

pub enum TimerCommand {
    Quit,
    Toggle,
}

pub trait TimerScreen {
    fn init() -> Result<Self> where Self: Sized;
    fn build_timer_screen(&mut self, time_elapsed: Duration) -> Result<()>;
    fn pop_cmd(&self) -> Result<Option<TimerCommand>>;
    fn flush(&mut self) -> Result<()>;
}

