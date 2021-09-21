mod brightness;
use brightness::Status;
mod error;
use error::Error;
pub mod operation;
use operation::Operation;

pub fn run(operation: Operation) -> Result<(), Error> {
    match operation {
        Operation::Get(path) => {
            let status = Status::get(path)?;
            println!("{}", status);
        }
        Operation::Set(path, brightness) => {
            let mut status = Status::get(path)?;
            status.set_brightness(brightness)?;
            status.save()?;
        }
    }
    Ok(())
}
