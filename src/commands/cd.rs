use crate::command::Command;
use std::env;
use std::error::Error;
use std::path::Path;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        let new_dir = args.get(0).map(|s| s.as_str()).unwrap_or("..");
        let root = Path::new(new_dir);
        env::set_current_dir(&root)?;
        Ok(())
    }
}