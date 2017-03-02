#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

mod backlight;
#[macro_use]
mod utils;

use std::process;

pub use errors::*;
pub use backlight::Backlight;
use utils::print_backtrace;

use clap::{App, Arg, SubCommand};



fn main() {
    let mut app = App::new("Backlight")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("set")
            .about("Set the brightness")
            .arg(Arg::with_name("percent").required(true)))
        .subcommand(SubCommand::with_name("inc")
            .about("Increment the current brightness")
            .arg(Arg::with_name("percent").required(true)))
        .subcommand(SubCommand::with_name("dec")
            .about("Decrement the current brightness")
            .arg(Arg::with_name("percent").required(true)))
        .subcommand(SubCommand::with_name("on").about("Turn the backlight on"))
        .subcommand(SubCommand::with_name("off").about("Turn the backlight off"))
        .subcommand(SubCommand::with_name("status").about("Get the backlight's current status"));
    let matches = app.clone().get_matches();

    let mut bl = unwrap_or_exit!(Backlight::new());

    match matches.subcommand() {
        ("set", Some(args)) => {
            let value: &str = args.value_of("percent").unwrap();
            let new_level = unwrap_or_exit!(value_to_int(value));
            unwrap_or_exit!(bl.set(new_level as usize));
        }
        ("inc", Some(args)) => {
            let value: &str = args.value_of("percent").unwrap();
            let new_level = unwrap_or_exit!(value_to_int(value));
            unwrap_or_exit!(bl.increment(new_level));
        }
        ("dec", Some(args)) => {
            let value: &str = args.value_of("percent").unwrap();
            let new_level = unwrap_or_exit!(value_to_int(value));
            unwrap_or_exit!(bl.increment(-1 * new_level));
        }

        ("on", _) => unwrap_or_exit!(bl.backlight_on(true)),
        ("off", _) => unwrap_or_exit!(bl.backlight_on(false)),
        ("status", _) => print_status(bl),

        _ => {
            app.print_help().expect("Couldn't print the help message");
            println!();
        }

    }
}

fn value_to_int(val: &str) -> Result<isize> {
    let has_sign = val.starts_with('-') || val.starts_with('+');
    let mut is_negative = false;
    let input = if has_sign {
        let mut characters = val.chars();
        is_negative = characters.next()
            .map(|c| c == '-')
            .ok_or_else(|| ErrorKind::NoInput)?;
        characters.collect()
    } else {
        val.to_string()
    };

    isize::from_str_radix(input.trim(), 10)
        .chain_err(|| format!("Converting {} to an integer failed", val))
        .map(|num| if is_negative { -1 * num } else { num })
}

fn print_status(b: Backlight) {
    let current = unwrap_or_exit!(b.current());
    let current_raw = unwrap_or_exit!(b.current_raw());
    let max = unwrap_or_exit!(b.max());
    println!("Brightness: {}/{} ({}%)", current_raw, max, current);
}

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            InvalidInteger(::std::num::ParseIntError);
        }

        errors {
            NoBacklightFound 
            InvalidBrightness
            NoInput
        }
    }
}
