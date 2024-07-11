use crate::command::Command;
use std::error::Error;
use std::process::Command as SystemCommand;

pub struct ExecuteCommand;

impl Command for ExecuteCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        if args.is_empty() {
            return Ok(());
        }
        let output = SystemCommand::new(&args[0])
            .args(&args[1..])
            .output()?;
        print!("{}", String::from_utf8_lossy(&output.stdout));
        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(())
    }
}