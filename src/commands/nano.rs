use crate::command::Command;
use std::error::Error;
use std::process::Command as SystemCommand;

pub struct NanoCommand;

impl Command for NanoCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        SystemCommand::new("nano")
            .args(args)
            .status()?;
        Ok(())
    }
}