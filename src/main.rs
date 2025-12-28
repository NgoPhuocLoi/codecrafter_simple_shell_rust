#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;
use crate::builtin::{echo::echo, check_type::check_type, execute_command::execute_command};

mod builtin;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    let built_in_commands = HashSet::from(["echo", "type", "exit"]);
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut command = input.trim();
        let mut remainder = "";

        if let Some(index) = command.find(" ") {
            remainder = &command[index+1..];
            command = &command[..index];
        }

        match command {
            "exit" => {
                return;
            },
            "echo" => {
                echo(remainder);
            },
            "type" => {
                check_type(remainder, &built_in_commands);
            }
            other => {
                execute_command(other, remainder);
            }
        }
    }
}
