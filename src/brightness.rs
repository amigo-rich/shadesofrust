use crate::error::Error;
use std::fmt;
use std::fs;
use std::path;

#[derive(Debug)]
pub struct Status {
    // What:        /sys/class/backlight/<backlight>/actual_brightness
    // Description: Show the actual brightness by querying the hardware.
    actual_brightness: u16,
    // What:        /sys/class/backlight/<backlight>/brightness
    // Description: Control the brightness for this <backlight>. Values
    //              are between 0 and max_brightness. This file will also
    //              show the brightness level stored in the driver, which
    //              may not be the actual brightness (see actual_brightness).
    brightness: u16,
    // What:        /sys/class/backlight/<backlight>/max_brightness
    // Description: Maximum brightness for <backlight>.
    max_brightness: u16,
    device_path: path::PathBuf,
}

impl Status {
    pub fn get(device_path: &path::Path) -> Result<Self, Error> {
        if !device_path.is_dir() {
            return Err(Error::DevicePathNotADirectory(device_path.to_path_buf()));
        }
        let actual_brightness =
            Status::file_content_to_u16(&device_path.join("actual_brightness"))?;
        let brightness = Status::file_content_to_u16(&device_path.join("brightness"))?;
        let max_brightness = Status::file_content_to_u16(&device_path.join("max_brightness"))?;
        Ok(Status {
            actual_brightness,
            brightness,
            max_brightness,
            device_path: device_path.to_path_buf(),
        })
    }
    pub fn save(&self) -> Result<(), Error> {
        let mut path = self.device_path.clone();
        path.push("brightness");
        let result = fs::write(&path, &self.brightness.to_string());
        if let Err(e) = result {
            return Err(Error::IO(e));
        }
        Ok(())
    }
    pub fn set_brightness(&mut self, input: &str) -> Result<(), Error> {
        let value = input
            .trim()
            .parse()
            .map_err(|error| Error::ParseToU16(input.to_string(), error))?;
        if value > self.max_brightness {
            return Err(Error::SetBrightnessInvalidValue(value, self.max_brightness));
        }
        self.brightness = value;
        Ok(())
    }
    fn file_content_to_u16(file_path: &path::Path) -> Result<u16, Error> {
        if !file_path.is_file() {
            return Err(Error::NotAFile(file_path.to_path_buf()));
        }
        let content = fs::read_to_string(file_path)?;
        let value = content
            .trim()
            .parse()
            .map_err(|error| Error::ParseToU16(content, error))?;
        Ok(value)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path_str_rep = self.device_path.to_str().unwrap_or("<path invalid>");
        write!(
            f,
            "Sysfs path: {}\nActual brightness: {}\nBrightness: {}\nMax brightness: {}",
            path_str_rep, self.actual_brightness, self.brightness, self.max_brightness
        )
    }
}
