use crate::builtin::{check_type::check_type, execute_command::execute_command};
use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{self, Write};

mod builtin;

fn get_args_from_arg_string(arg_str: &str) -> Vec<String> {
    shlex::split(arg_str).unwrap()
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut command = input.trim();
        let mut remainder = "";

        let mut first_char = "";

        if command.len() > 1 {
            first_char = &command[..1].trim();
        }

        match first_char {
            "'" => {
                let i = &command[1..].find("'").unwrap() + 1;
                remainder = &command[i + 1..];
                command = &command[1..i];
            }
            "\"" => {
                let i = &command[1..].find("\"").unwrap() + 1;
                remainder = &command[i + 1..];
                command = &command[1..i];
            }
            _ => {
                if let Some(index) = command.find(" ") {
                    remainder = &command[index + 1..];
                    command = &command[..index];
                }
            }
        }

        let args = get_args_from_arg_string(remainder.trim());

        match command {
            "exit" => {
                return;
            }
            "type" => {
                let built_in_commands = HashSet::from(["echo", "type", "exit"]);
                check_type(remainder, &built_in_commands);
            }
            "" => {}
            other => {
                execute_command(other, args);
            }
        }
    }
}
