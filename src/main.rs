use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::{Config, Context, Editor, Helper};

use crate::builtin::check_type::fin_executable_paths_like;
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
    type Candidate = String;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        let commands = vec!["echo", "exit"];
        let mut candidates = vec![];
        let input_command = &line[..pos].trim();
        for cmd in commands {
            if cmd.starts_with(input_command) {
                candidates.push(format!("{cmd}"));
            }
        }

        if let Ok(commands_in_path) = fin_executable_paths_like(input_command) {
            for cmd in commands_in_path {
                let s = cmd.file_name().unwrap().to_str().unwrap();
                candidates.push(format!("{s}"));
            }
        }
        candidates.sort();
        candidates.dedup();

        if candidates.len() == 1 {
            candidates = vec![format!("{} ", &candidates[0])];
        }

        Ok((0, candidates))
    }
}

fn main() {
    let config = Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build();
    let mut rl = Editor::with_config(config).unwrap();

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
