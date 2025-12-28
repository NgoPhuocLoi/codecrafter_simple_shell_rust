#[allow(unused_imports)]
use std::io::{self, Write};

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
            other => {
                println!("{}: command not found", other)
            }
        }
    }
}
