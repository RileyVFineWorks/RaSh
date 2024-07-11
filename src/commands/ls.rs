use crate::command::Command;
use std::error::Error;
use std::process::Command as SystemCommand;

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        let output = SystemCommand::new("ls")
            .args(args)
            .output()?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}