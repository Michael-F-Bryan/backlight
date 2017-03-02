#[macro_use]
extern crate error_chain;
extern crate notify_rust;
extern crate clap;

mod backlight;
#[macro_use]
mod utils;

use std::process;

pub use errors::*;
pub use backlight::Backlight;
use utils::{send_notification, print_backtrace};



fn main() {
    let mut bl = unwrap_or_exit!(Backlight::new());
    let current = unwrap_or_exit!(bl.current());

    unwrap_or_exit!(bl.backlight_on(true));

    println!("Current brightness: {:?}", current);
    send_notification(format!("Current value: {}%", current));


    // unwrap_or_exit!(bl.set_absolute(208));
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
        }
    }
}
