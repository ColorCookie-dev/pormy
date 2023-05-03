use std::{
    fmt::Display,
    io::{stdout, Stdout, Write},
};
use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
};

pub struct RawTerm<T>
where
    T: Write,
{
    out: RawTerminal<T>,
}

impl<T: Write> RawTerm<T> {
    pub fn new(out: T) -> Result<Self, std::io::Error> {
        Ok(Self {
            out: out.into_raw_mode()?,
        })
    }

    pub fn get_out(&mut self) -> &mut RawTerminal<T> {
        &mut self.out
    }

    pub fn set_color(&mut self, color: impl color::Color) -> Result<(), std::io::Error> {
        write!(self.out, "{}", color::Fg(color))
    }

    pub fn write_raw_line(&mut self, show: impl Display, line: u16) -> Result<(), std::io::Error> {
        write!(
            self.out,
            "{goto}{show}",
            goto = cursor::Goto(1, line),
            show = show
        )
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        write!(self.out, "{}", clear::All)
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.out.flush()
    }

    pub fn reset(&mut self) -> Result<(), std::io::Error> {
        write!(
            self.out,
            "{top}{clear}{show}{color}",
            top = cursor::Goto(1, 1),
            clear = clear::All,
            color = color::Fg(color::Reset),
            show = termion::cursor::Show
        )?;

        self.out.flush()
    }
}

impl Default for RawTerm<Stdout> {
    fn default() -> Self {
        Self {
            out: stdout().into_raw_mode().unwrap(),
        }
    }
}

impl<T: Write> Drop for RawTerm<T> {
    fn drop(&mut self) {
        self.reset().ok();
    }
}
