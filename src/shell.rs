use std::collections::HashMap;
use std::error::Error;
use std::process::Command as SystemCommand;

use crate::command::Command;
use crate::commands::*;

pub struct Shell {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Shell {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        commands.insert("cd".to_string(), Box::new(cd::CdCommand) as Box<dyn Command>);
        commands.insert("ls".to_string(), Box::new(ls::LsCommand) as Box<dyn Command>);
        commands.insert("pwd".to_string(), Box::new(pwd::PwdCommand) as Box<dyn Command>);
        commands.insert("cat".to_string(), Box::new(cat::CatCommand) as Box<dyn Command>);
        commands.insert("vim".to_string(), Box::new(vim::VimCommand) as Box<dyn Command>);
        commands.insert("nano".to_string(), Box::new(nano::NanoCommand) as Box<dyn Command>);
        Self { commands }
    }

    pub fn execute_command(&self, command: &str, args: &[String]) -> Result<(), Box<dyn Error>> {
        match self.commands.get(command) {
            Some(cmd) => cmd.execute(args),
            None => {
                if !command.is_empty() {
                    self.execute_system_command(command, args)
                } else {
                    Ok(()) // Empty command, do nothing
                }
            }
        }
    }

    fn execute_system_command(&self, command: &str, args: &[String]) -> Result<(), Box<dyn Error>> {
        let output = SystemCommand::new(command)
            .args(args)
            .output()?;

        if output.status.success() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
            Ok(())
        } else {
            Err(format!("Command failed: {}", String::from_utf8_lossy(&output.stderr)).into())
        }
    }

    pub fn get_command_names(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }
}