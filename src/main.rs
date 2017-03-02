#[macro_use]
extern crate error_chain;
extern crate clap;

mod backlight;

use std::process;
use std::io::{self, Write};

pub use errors::*;
pub use backlight::Backlight;


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


fn main() {
    let bl = unwrap_or_exit!(Backlight::new());
    let max = unwrap_or_exit!(bl.max());

    println!("Max brightness: {:?}", max);
}


mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            InvalidInteger(::std::num::ParseIntError);
        }

        errors {
            NoBacklightFound {
                description("No backlight found")
                display("No backlight found")
            }
        }
    }
}

fn print_backtrace(e: Error) {
    let mut stderr = io::stderr();
    writeln!(stderr, "error: {}", e);

    for e in e.iter().skip(1) {
        writeln!(stderr, "caused by: {}", e);
    }
}
