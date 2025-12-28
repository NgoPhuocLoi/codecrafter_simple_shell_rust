#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;

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
                println!("{remainder}");
            },
            "type" => {
                let is_built_in = built_in_commands.contains(&remainder);
                if is_built_in {
                    println!("{command} is a shell builtin");
                } else {
                    println!("{remainder}: command not found");
                }
            }
            other => {
                println!("{}: command not found", other);
            }
        }
    }
}
