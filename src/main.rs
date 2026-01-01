use crate::builtin::{echo::echo, execute_command::execute_command};
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

        if let Some(index) = command.find(" ") {
            remainder = &command[index + 1..];
            command = &command[..index];
        }

        let args = get_args_from_arg_string(remainder);

        match command {
            "exit" => {
                return;
            }
            // "echo" => {
            //     echo(args);
            // }
            // "type" => {
            //     check_type(remainder, &built_in_commands);
            // }
            other => {
                execute_command(other, args);
            }
        }
    }
}
