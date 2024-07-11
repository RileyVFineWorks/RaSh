use rustyline::error::ReadlineError;
use std::env;
use std::process::Command;

mod commands;

fn main() -> rustyline::Result<()> {
    let mut rl = rustyline::DefaultEditor::new()?;
    loop {
        let current_dir = env::current_dir().unwrap();
        let prompt = format!("{} $ ", current_dir.display());

        match rl.readline(&prompt) {
            Ok(line) => {
                let mut parts = line.trim().split_whitespace();
                let command = parts.next().unwrap_or("");
                let args: Vec<&str> = parts.collect();

                match command {
                    "cd" => {
                        commands::cd(&args);
                    }
                    "vim" => {
                        commands::open_file("vim", &args);
                    }
                    "nano" => {
                        commands::open_file("nano", &args);
                    }
                    "ls" => {
                        let output = Command::new("ls")
                            .args(&args)
                            .output()
                            .unwrap_or_else(|_| panic!("Failed to execute command: ls"));
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                    "pwd" => {
                        let current_dir = env::current_dir().unwrap();
                        println!("{}", current_dir.display());
                    }
                    "cat" => {
                        let output = Command::new("cat")
                            .args(&args)
                            .output()
                            .unwrap_or_else(|_| panic!("Failed to execute command: cat"));
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                    "exit" => {
                        println!("Exiting the shell...");
                        break;
                    }
                    "" => {
                        // Empty command, do nothing
                    }
                    _ => {
                        commands::execute_command(command, &args);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted. Exiting the shell...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting the shell...");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}