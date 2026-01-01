use super::check_type::find_executable_path;
use std::io::{self, Write};
use std::process::Command;

pub fn execute_command(command: &str, args: Vec<String>) {
    match find_executable_path(command) {
        Some(_) => {
            let output = Command::new(command)
                .args(args.iter().filter(|&i| i != " "))
                .output()
                .expect("Failed to execute command");
            io::stdout().write_all(&output.stdout).expect(" Failed");
            io::stderr().write_all(&output.stderr).expect(" Failed");
        }
        None => {
            println!("{}: not found", command);
        }
    }
}
