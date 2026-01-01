use crate::builtin::{echo::echo, execute_command::execute_command};
#[allow(unused_imports)]
use std::io::{self, Write};

mod builtin;

// .filter(|i| i.trim() != "")

fn remove_redundant_spaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn normalize_arg_str(arg_str: &str) -> Vec<String> {
    let args: Vec<&str> = arg_str.split("'").collect();
    let l = args.len();

    args.iter()
        .enumerate()
        .map(|(i, _)| {
            let is_first_agr = i == 0 && args[i] != "";
            let is_last_agr = i == l - 1 && args[l - 1] != "";
            let is_middle_agr =
                i > 1 && i < l - 1 && args[i] != "" && args[i - 1] == "" && args[i + 1] == "";
            if is_first_agr || is_last_agr || is_middle_agr {
                remove_redundant_spaces(args[i])
            } else {
                args[i].to_string()
            }
        })
        .filter(|i| i.trim_matches(|c: char| c.is_whitespace() && c != ' ') != "")
        .collect()
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

        let args = normalize_arg_str(remainder);

        match command {
            "exit" => {
                return;
            }
            "echo" => {
                echo(args);
            }
            // "type" => {
            //     check_type(remainder, &built_in_commands);
            // }
            other => {
                execute_command(other, args);
            }
        }
    }
}
