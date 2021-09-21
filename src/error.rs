use std::fmt;
use std::path;

#[derive(Debug)]
pub enum Error {
    DevicePathNotADirectory(path::PathBuf),
    IO(std::io::Error),
    NotAFile(path::PathBuf),
    ParseToU16(String, std::num::ParseIntError),
    SetBrightnessInvalidValue(u16, u16),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DevicePathNotADirectory(path) => {
                let path_str_rep = path.to_str().unwrap_or("<path invalid>");
                write!(
                    f,
                    "Sysfs path: '{}' is not a directory or has invalid permissions",
                    path_str_rep
                )
            }
            Error::IO(e) => {
                write!(f, "While reading or writing, an IO Error occurred: '{}'", e)
            }
            Error::NotAFile(path) => {
                let path_str_rep = path.to_str().unwrap_or("<path invalid>");
                write!(
                    f,
                    "Sysfs path: '{}' is not a file or has invalid permissions",
                    path_str_rep
                )
            }
            Error::ParseToU16(input, e) => {
                write!(
                    f,
                    "While parsing: '{}' as a u16 value, an error occured: '{}'",
                    input, e
                )
            }
            Error::SetBrightnessInvalidValue(requested, maximum) => {
                write!(
                    f,
                    "The requested brightness: '{}' is greater than the maximum value: '{}'",
                    requested, maximum
                )
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}
