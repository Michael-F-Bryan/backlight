use errors::*;
use std::fs::{self, DirEntry, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::num::ParseIntError;


pub struct Backlight {
    dir: PathBuf,
}

impl Backlight {
    pub fn new() -> Result<Backlight> {
        let dir = find_backlight_directory()?;
        Ok(Backlight { dir: dir })
    }

    pub fn max(&self) -> Result<i64> {
        let mut path = self.dir.clone();
        path.push("max_brightness");

        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;

        i64::from_str_radix(buf.trim(), 10).map_err(|e| e.into())
    }
}

fn find_backlight_directory() -> Result<PathBuf> {
    let mut dir = fs::read_dir("/sys/class/backlight")
        .chain_err(|| "Couldn't read the root backlight directory")?;

    match dir.next() {
        Some(Ok(first)) => Ok(first.path()),
        Some(Err(e)) => Err(e.into()),
        None => Err(ErrorKind::NoBacklightFound.into()),
    }
}
