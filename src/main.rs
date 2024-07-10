use std::env;
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod commands;
mod utils;

fn main() {
    let mut rl = Editor::<()>::new();
    if let Err(err) = rl {
        match err {
            ReadlineError::Io(err) => {
                eprintln!("Failed to create readline editor: {}", err);
                std::process::exit(1);
            }
            ReadlineError::Eof => {
                eprintln!("Reached end of file");
                std::process::exit(0);
            }
            ReadlineError::Interrupted => {
                eprintln!("Interrupted");
                std::process::exit(1);
            }
            _ => {
                eprintln!("Unknown readline error");
                std::process::exit(1);
            }
        }
    }
    let mut rl = rl.unwrap();
    let history_file = utils::get_history_file_path();
    if rl.load_history(&history_file).is_err() {
        println!("No previous history.");
    }

    loop {
        let current_dir = env::current_dir().unwrap();
        let prompt = format!("{}> ", current_dir.display());

        match rl.readline(&prompt) {
            Ok(input) => {
                rl.add_history_entry(input.as_str());

                let mut parts = input.trim().split_whitespace();
                let command = parts.next().unwrap_or("");
                let args = parts.collect::<Vec<&str>>();

                match command {
                    "cd" => commands::cd(&args),
                    "vim" | "nano" | "emacs" => commands::open_file(command, &args),
                    _ => commands::execute_command(command, &args),
                }
            }
            Err(_) => {
                println!("Exiting...");
                break;
            }
        }
    }

    rl.save_history(&history_file).unwrap();
}