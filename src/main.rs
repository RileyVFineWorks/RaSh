use rustyline::error::ReadlineError;
use std::env;
use std::error::Error;

mod command;
mod commands;
mod shell;

use shell::Shell;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = rustyline::DefaultEditor::new()?;
    let shell = Shell::new();

    // Load command history
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let current_dir = env::current_dir()?;
        let prompt = format!("{} $ ", current_dir.display());

        match rl.readline(&prompt) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let mut parts = line.trim().split_whitespace();
                let command = parts.next().unwrap_or("");
                let args: Vec<String> = parts.map(String::from).collect();

                if command == "exit" {
                    println!("Exiting the shell...");
                    break;
                }

                if let Err(e) = shell.execute_command(command, &args) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save command history
    rl.save_history("history.txt")?;

    Ok(())
}