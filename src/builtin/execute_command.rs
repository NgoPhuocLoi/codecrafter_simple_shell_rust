use super::check_type::find_executable_path;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn execute_command(command: &str, args: Vec<String>) {
    let redirect_stdout_sympol = String::from(">");
    let redirect_stdout_explicit_sympol = String::from("1>");
    let redirect_stderr_sympol = String::from("2>");
    match find_executable_path(command) {
        Some(_) => {
            let mut is_redirect_stdout = false;
            let mut is_redirect_stderr = false;
            for i in &args {
                if *i == redirect_stdout_sympol || *i == redirect_stdout_explicit_sympol {
                    is_redirect_stdout = true;
                    break;
                }
                if *i == redirect_stderr_sympol {
                    is_redirect_stderr = true;
                }
            }
            let mut arg_list = &args[..];
            let mut redirected_file_path = &String::from("");

            if is_redirect_stdout {
                let mut iter = args.split(|i| {
                    *i == redirect_stdout_sympol || *i == redirect_stdout_explicit_sympol
                });
                arg_list = iter.next().unwrap();
                redirected_file_path = &iter.next().unwrap()[0];
            } else if is_redirect_stderr {
                let mut iter = args.split(|i| *i == redirect_stderr_sympol);
                arg_list = iter.next().unwrap();
                redirected_file_path = &iter.next().unwrap()[0];
            }

            let output = Command::new(command)
                .args(arg_list)
                .output()
                .expect("Failed to execute command");

            if !is_redirect_stdout {
                io::stdout().write_all(&output.stdout).expect(" Failed");
            } else if is_redirect_stdout && redirected_file_path != "" {
                let p = Path::new(redirected_file_path);
                match fs::OpenOptions::new().write(true).open(p) {
                    Ok(mut f) => {
                        f.write_all(&output.stdout).unwrap();
                    }
                    Err(_) => {
                        let mut created_file = fs::File::create(p).unwrap();
                        created_file.write_all(&output.stdout).unwrap();
                    }
                }
            }

            if !is_redirect_stderr {
                io::stderr().write_all(&output.stderr).expect(" Failed");
            } else if is_redirect_stderr && redirected_file_path != "" {
                let p = Path::new(redirected_file_path);
                match fs::OpenOptions::new().write(true).open(p) {
                    Ok(mut f) => {
                        f.write_all(&output.stderr).unwrap();
                    }
                    Err(_) => {
                        let mut created_file = fs::File::create(p).unwrap();
                        created_file.write_all(&output.stderr).unwrap();
                    }
                }
            }
        }
        None => {
            println!("{}: not found", command);
        }
    }
}
