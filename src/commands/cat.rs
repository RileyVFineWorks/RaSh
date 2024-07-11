use crate::command::Command;
use std::error::Error;
use std::process::Command as SystemCommand;

pub struct CatCommand;

impl Command for CatCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        let output = SystemCommand::new("cat")
            .args(args)
            .output()?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}