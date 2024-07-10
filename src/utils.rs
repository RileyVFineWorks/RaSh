use std::env;
use std::path::PathBuf;

pub fn get_history_file_path() -> PathBuf {
    let mut history_file = PathBuf::from(env::var("HOME").unwrap_or_else(|_| "/".to_string()));
    history_file.push(".rust_shell_history");
    history_file
}