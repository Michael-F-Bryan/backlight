use errors::*;
use std::fs;
use std::path::PathBuf;

use utils::{write_int_to_file, read_int_from_file, send_notification};


/// The object in charge of manipulating your laptop's backlight.
pub struct Backlight {
    dir: PathBuf,
}

impl Backlight {
    /// Get a new handle to your laptop's backlight.
    pub fn new() -> Result<Backlight> {
        let dir = find_backlight_directory()?;
        Ok(Backlight { dir: dir })
    }

    /// The maximum value your backlight can be set to (units are completely
    /// arbitrary).
    pub fn max(&self) -> Result<usize> {
        let mut path = self.dir.clone();
        path.push("max_brightness");
        read_int_from_file(path)
    }

    /// The current backlight level (units are completely arbitrary).
    pub fn current_raw(&self) -> Result<usize> {
        let mut path = self.dir.clone();
        path.push("actual_brightness");
        read_int_from_file(path)
    }

    /// Set the backlight to some value between 0 and max().
    pub fn set_absolute(&mut self, value: usize) -> Result<()> {
        if value > self.max()? {
            Err(ErrorKind::InvalidBrightness.into())
        } else {
            let mut path = self.dir.clone();
            path.push("brightness");
            write_int_to_file(path, value).chain_err(|| "Writing to the backlight file failed")
        }
    }

    /// Get the current brightness level rounded to the nearest percent.
    pub fn current(&self) -> Result<usize> {
        Ok(self.current_raw()? * 100 / self.max()?)
    }

    pub fn set(&mut self, value: usize) -> Result<()> {
        let raw = self.max()? * value / 100;
        self.set_absolute(raw)
    }

    pub fn backlight_on(&mut self, on: bool) -> Result<()> {
        let mut path = self.dir.clone();
        path.push("bl_power");

        let value = if on { 0 } else { 1 };
        write_int_to_file(path, value)
    }

    fn increment_raw(&mut self, amount: isize) -> Result<()> {
        let current = self.current_raw()? as isize;
        let value = current + amount;
        let max = self.max()?;
        if value >= max as isize {
            self.set(max)
        } else if value <= 0 {
            self.set(0)
        } else {
            self.set(value as usize)
        }
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
