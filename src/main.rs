use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::{Context, Editor, Helper};

use crate::builtin::check_type::find_executable_path_like;
use crate::builtin::{check_type::check_type, execute_command::execute_command};
use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{self, Write};

mod builtin;

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
        _: &Context<'_>,
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

        if let Ok(Some(command_in_path)) = find_executable_path_like(&line[..pos]) {
            if let Some(file_name) = command_in_path.file_name().and_then(|name| name.to_str()) {
                candidates.push(Pair {
                    display: format!("{file_name} "),
                    replacement: format!("{file_name} "),
                })
            }
        }

        Ok((0, candidates))
    }
}

fn main() {
    let mut rl = Editor::new().unwrap();

    rl.set_helper(Some(MyHelper));
    loop {
        let input = rl.readline("$ ").unwrap();
        let parsed_input = shlex::split(input.trim()).unwrap();
        let command = parsed_input[0].as_str();
        let args = &parsed_input[1..].to_vec();

        match command {
            "exit" => {
                return;
            }
            "type" => {
                let built_in_commands = HashSet::from(["echo", "type", "exit"]);
                check_type(&args.join(""), &built_in_commands);
            }
            "" => {}
            other_command => {
                execute_command(other_command, args);
            }
        }
    }
}
