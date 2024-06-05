use crate::fmt;
use crate::fmt::Write;

use crate::io::stdout::Stdout;
use crate::Mutex;

type Stderr = Stdout;

pub static STDERR: Mutex<Stderr> = Mutex::new(Stderr::new());

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    let mut stderr = STDERR.lock();
    let _ = stderr.write_fmt(args);
}

pub fn add_sink(sink: &'static Mutex<dyn Write>) {
    let mut stderr = STDERR.lock();

    stderr.add_sink(sink);
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::io::stderr::_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        $crate::eprintln!("[{}:{}:{}]", core::file!(), core::line!(), core::column!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::eprintln!("[{}:{}:{}] {} = {:#?}",
                    core::file!(), core::line!(), core::column!(), core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
