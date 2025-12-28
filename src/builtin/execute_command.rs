use super::check_type::find_executable_path;
use std::process::Command;
use std::io::{self, Write};

pub fn execute_command(command: &str, remainder: &str) {
    match find_executable_path(command) {
        Some(_) => {
            let output = Command::new(command)
                                .arg(remainder)
                                .output()
                                .expect("Failed to execute command");
            io::stdout().write_all(&output.stdout).expect(" Failed");
            io::stderr().write_all(&output.stderr).expect(" Failed");
        },
        None => {
            println!("{}: not found", command);
        }
    }
}