use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use errors::*;


/// Unwraps a result. If there was an error, print the backtrace and then
/// exit.
macro_rules! unwrap_or_exit {
    ($maybe_err:expr) => {
        unwrap_or_exit!($maybe_err, 1)
    };

    ($maybe_err:expr, $status_code:expr) => {
        match $maybe_err {
            Ok(thing) => thing,
            Err(e) => {
                print_backtrace(e);
                process::exit($status_code);
            }
        }
    };
}

pub fn write_int_to_file<P: AsRef<Path>>(path: P, value: usize) -> Result<()> {
    let mut f = OpenOptions::new().write(true)
        .truncate(true)
        .open(path)
        .chain_err(|| "Couldn't open the provided path for writing")?;
    write!(f, "{}", value)?;
    Ok(())
}



/// Small helper for reading a file which contains a single integer.
pub fn read_int_from_file<P: AsRef<Path>>(path: P) -> Result<usize> {
    let mut f = File::open(path).chain_err(|| "Couldn't open the provided path for reading")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    usize::from_str_radix(buf.trim(), 10).map_err(|e| e.into())
}


/// Given an error_chain Error, print the error and everything that caused it.
pub fn print_backtrace(e: Error) {
    let mut stderr = io::stderr();
    writeln!(stderr, "error: {}", e).expect("Writing to stderr failed");

    for e in e.iter().skip(1) {
        writeln!(stderr, "\tcaused by: {}", e).expect("Writing to stderr failed");
    }
}
