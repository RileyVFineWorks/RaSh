use crate::command::Command;
use std::env;
use std::error::Error;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn execute(&self, _args: &[String]) -> Result<(), Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        println!("{}", current_dir.display());
        Ok(())
    }
}