use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts.collect::<Vec<&str>>();

        match command {
            "cd" => {
                if args.is_empty() {
                    // If no arguments are provided, change to the home directory
                    let home_dir = env::var("HOME").unwrap_or_else(|_| "/".to_string());
                    env::set_current_dir(Path::new(&home_dir)).unwrap();
                } else {
                    // Change to the specified directory
                    let path = args[0];
                    env::set_current_dir(Path::new(path)).unwrap_or_else(|_| {
                        eprintln!("cd: no such file or directory: {}", path);
                    });
                }
            }
            "vim" | "nano" | "emacs" => {
                if args.is_empty() {
                    eprintln!("{}: please provide a file name", command);
                } else {
                    let file_path = args[0];
                    let output = Command::new(command)
                        .arg(file_path)
                        .status()
                        .unwrap_or_else(|_| panic!("Failed to execute command: {}", command));

                    if !output.success() {
                        eprintln!("Failed to open file with {}", command);
                    }
                }
            }
            _ => {
                let mut command_path = PathBuf::from("/usr/bin");
                command_path.push(command);

                if command_path.exists() {
                    let output = Command::new(command)
                        .args(args)
                        .output()
                        .unwrap_or_else(|_| panic!("Failed to execute command: {}", command));

                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    eprintln!("{}: command not found", command);
                }
            }
        }
    }
}