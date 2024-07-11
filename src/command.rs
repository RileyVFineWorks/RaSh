use std::error::Error;

pub trait Command {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn Error>>;
}