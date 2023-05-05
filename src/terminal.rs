pub enum TimerCommand {
    Quit,
    Toggle,
}

#[derive(Debug)]
pub enum CommandRecvError {
    IOError(std::io::Error),
    TryRecvError(std::sync::mpsc::TryRecvError),
    NotACommand,
}

impl std::fmt::Display for CommandRecvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "{}", err),
            Self::TryRecvError(err) => write!(f, "{}", err),
            Self::NotACommand =>
                write!(f, "The key pressed is not a command"),
        }
    }
}

impl From<std::io::Error> for CommandRecvError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<std::sync::mpsc::TryRecvError> for CommandRecvError {
    fn from(err: std::sync::mpsc::TryRecvError) -> Self {
        Self::TryRecvError(err)
    }
}

impl std::error::Error for CommandRecvError { }

pub trait TimerScreen {
    fn init() -> Result<Self, std::io::Error> where Self: Sized;
    fn build_timer_screen(&mut self, time_elapsed: std::time::Duration)
        -> Result<(), std::io::Error>;
    fn pop_cmd(&self) -> Result<TimerCommand, CommandRecvError>;
    fn flush(&mut self) -> Result<(), std::io::Error>;
}

