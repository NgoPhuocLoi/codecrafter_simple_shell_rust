use is_executable::IsExecutable;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn is_command_at_path(command: &str, path: &Path) -> bool {
    if path.ends_with(command) && path.is_executable() {
        return true;
    }
    false
}

pub fn find_executable_path(command: &str) -> Option<PathBuf> {
    if let Some(paths) = env::var_os("PATH") {
        for path_buf in std::env::split_paths(&paths) {
            let p = path_buf.as_path();
            if p.is_file() && is_command_at_path(command, &p) {
                return Some(path_buf);
            }

            if p.is_dir() {
                for entry in fs::read_dir(p).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    if is_command_at_path(command, &path) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

pub fn check_type(command: &str, built_in_commands: &HashSet<&str>) {
    let is_built_in = built_in_commands.contains(&command);
    if is_built_in {
        println!("{command} is a shell builtin");
        return;
    }

    match find_executable_path(command) {
        Some(path) => println!("{}", path.display()),
        None => println!("{}: not found", command),
    }
}
