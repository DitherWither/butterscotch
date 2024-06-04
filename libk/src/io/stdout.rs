use crate::fmt;
use crate::fmt::Write;
use crate::io::{self};
use crate::vec::Vec;

use crate::{utils, Mutex};

pub static STDOUT: Mutex<Stdout> = Mutex::new(Stdout::new());

pub struct Stdout {
    sinks: Vec<&'static Mutex<dyn Write>>,
}

unsafe impl Send for Stdout {}
unsafe impl Sync for Stdout {}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // TODO instead of instantly exiting on error, try all sinks
        // and return afterwards. This way, if one sink misbehaves,
        // the other sinks will still get the logs
        for sink in &mut self.sinks {
            sink.lock().write_str(s)?;
        }
        Ok(())
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let str = utils::string_from_u8_nul_utf(buf).map_err(|_| io::Error::InvalidString)?;
        self.write_str(&str).map_err(|_| io::Error::WriteError)?;
        Ok(str.len())
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        // Empty for now, as we haven't implemented any form of buffering
        Ok(())
    }
}

impl Stdout {
    pub(crate) const fn new() -> Self {
        Self { sinks: Vec::new() }
    }

    pub(crate) fn add_sink(&mut self, sink: &'static Mutex<dyn Write>) {
        self.sinks.push(sink);
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut stdout = STDOUT.lock();
    let _ = stdout.write_fmt(args);
}

pub fn add_sink(sink: &'static Mutex<dyn Write>) {
    let mut stdout = STDOUT.lock();

    stdout.add_sink(sink);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::stdout::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
