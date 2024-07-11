# Rash
- Honestly just wanting to play around with rust to see if I can replace my current terminal emulator
- its like bash or fish or zsh but rash
- as of now has basic bash commands (cd/cat/ls/pwd/etc) and can also be used for text editor of choice

## Installation
1. Make sure you have Rust and Cargo installed. If not, install them from https://rustup.rs/
2. Clone this repository: https://github.com/RileyVFineWorks/RaSh.git
3. Navigate to the project directory: `cd rash`
4. Install the shell: cargo install --path .
5. Ensure that `~/.cargo/bin` is in your PATH.

## Usage
After installation, you can start the shell from anywhere by running: rash

## Optional setup for default shell
1. This repo also contains the shell script to set this as your default shell in your terminal emulator of choice
2. running `chmod _x optional.sh` will create the .exe that users can then move into their desired path
3. In the shell section of your terminal emulator feel free to pass in the .exe
4. Example from my alacritty config :

```toml
[font]
size = 14

[font.normal]
family = "Fira Code"
style = "Regular"

[shell]
program = 'C:\Users\RashUser\.cargo\bin\rash.exe -c "cd; exec rash"'

[window]
opacity = 0.95
