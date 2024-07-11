use std::env;
use std::path::Path;
use std::process::Command;

pub fn cd(args: &[&str]) {
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

pub fn open_file(command: &str, args: &[&str]) {
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

pub fn execute_command(command: &str, args: &[&str]) {
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let command_path = Path::new(dir).join(command);
            if command_path.exists() {
                let output = Command::new(command)
                    .args(args)
                    .output()
                    .unwrap_or_else(|_| panic!("Failed to execute command: {}", command));
                println!("{}", String::from_utf8_lossy(&output.stdout));
                return;
            }
        }
    }
    eprintln!("{}: command not found", command);
}