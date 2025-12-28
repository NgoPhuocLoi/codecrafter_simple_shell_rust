#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}: command not found", input.trim());
        },
        Err(_) => {
            println!("Failed to read the command!");
        }
    }
    
}
