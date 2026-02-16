use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::{Context, DefaultEditor, Editor, Helper};

use crate::builtin::{check_type::check_type, execute_command::execute_command};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashSet, io::Read};

mod builtin;

fn get_args_from_arg_string(arg_str: &str) -> Vec<String> {
    shlex::split(arg_str).unwrap()
}

struct MyHelper;
impl rustyline::validate::Validator for MyHelper {}
impl rustyline::hint::Hinter for MyHelper {
    type Hint = String;
}
impl rustyline::highlight::Highlighter for MyHelper {}
impl Helper for MyHelper {}
impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        let commands = vec!["echo", "exit"];
        let mut candidates = vec![];

        for cmd in commands {
            if cmd.starts_with(&line[..pos]) {
                candidates.push(Pair {
                    display: format!("{cmd} "),
                    replacement: format!("{cmd} "),
                })
            }
        }
        Ok((0, candidates))
    }
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    let mut rl = Editor::new().unwrap();

    rl.set_helper(Some(MyHelper));
    loop {
        let input = rl.readline("$ ").unwrap();
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
