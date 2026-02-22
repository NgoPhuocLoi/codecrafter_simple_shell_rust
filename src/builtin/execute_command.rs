use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

use super::check_type::find_executable_path;

#[derive(Debug, PartialEq, Eq)]
enum OutputCode {
    NoRedirect,
    RedirectStdout,
    RedirectStderr,
    AppendStdout,
    AppendStderr,
}

impl OutputCode {
    fn is_redirect_stdout(&self) -> bool {
        return *self == Self::RedirectStdout || *self == Self::AppendStdout;
    }

    fn is_redirect_stderr(&self) -> bool {
        return *self == Self::RedirectStderr || *self == Self::AppendStderr;
    }
}

impl From<&String> for OutputCode {
    fn from(s: &String) -> Self {
        let redirect_stdout_sympol = String::from(">");
        let redirect_stdout_explicit_sympol = String::from("1>");
        let redirect_stderr_sympol = String::from("2>");
        let append_stdout_sympol = String::from(">>");
        let append_stdout_explicit_sympol = String::from("1>>");
        let append_stderr_sympol = String::from("2>>");

        if *s == redirect_stdout_sympol || *s == redirect_stdout_explicit_sympol {
            return Self::RedirectStdout;
        }

        if *s == redirect_stderr_sympol {
            return Self::RedirectStderr;
        }

        if *s == append_stdout_sympol || *s == append_stdout_explicit_sympol {
            return Self::AppendStdout;
        }

        if *s == append_stderr_sympol {
            return Self::AppendStderr;
        }

        return Self::NoRedirect;
    }
}

pub fn execute_command(command: &str, args: &Vec<String>) {
    match find_executable_path(command) {
        Ok(Some(_)) => {
            let mut arg_list = &args[..];
            let mut output_codes: Vec<OutputCode> = Vec::new();
            let mut output_code_indices: Vec<usize> = Vec::new();

            let mut i = 0;
            while i < args.len() {
                match OutputCode::from(&args[i]) {
                    OutputCode::NoRedirect => {
                        i += 1;
                    }
                    other => {
                        if output_codes.is_empty() {
                            arg_list = &args[..i];
                        }
                        output_codes.push(other);
                        output_code_indices.push(i);
                        i += 1;
                    }
                }
            }

            let output = Command::new(command).args(arg_list).output().unwrap();

            let mut is_redirect_stdout = false;
            let mut is_redirect_stderr = false;

            let mut i = 0;

            while i < output_codes.len() {
                let code = &output_codes[i];
                let code_index = output_code_indices[i];
                let mut file_path_str = "";
                if code_index + 1 < args.len() {
                    file_path_str = &args[code_index + 1];
                }

                if code.is_redirect_stderr() {
                    is_redirect_stderr = true;
                }

                if code.is_redirect_stdout() {
                    is_redirect_stdout = true;
                }

                match code {
                    OutputCode::RedirectStdout => {
                        fs::write(file_path_str, &output.stdout).unwrap();
                    }
                    OutputCode::RedirectStderr => {
                        fs::write(file_path_str, &output.stderr).unwrap();
                    }
                    OutputCode::AppendStdout => {
                        fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(Path::new(file_path_str))
                            .unwrap()
                            .write_all(&output.stdout)
                            .unwrap();
                    }
                    OutputCode::AppendStderr => {
                        fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(Path::new(file_path_str))
                            .unwrap()
                            .write_all(&output.stderr)
                            .unwrap();
                    }
                    _ => {}
                }

                i += 1;
            }

            if !is_redirect_stdout {
                io::stdout().write_all(&output.stdout).unwrap();
            }

            if !is_redirect_stderr {
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
        Ok(None) => {
            println!("{}: not found", command);
        }
        Err(e) => {
            eprintln!("Error searching for command: {}", e);
        }
    }
}
