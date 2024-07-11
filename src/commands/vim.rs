use crate::command::Command;
use std::error::Error;
use std::process::Command as SystemCommand;

pub struct VimCommand;

impl Command for VimCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        SystemCommand::new("vim")
            .args(args)
            .status()?;
        Ok(())
    }
}