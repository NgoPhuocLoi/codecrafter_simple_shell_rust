
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::io;
use is_executable::IsExecutable;

fn is_command_at_path(command: &str, path: &Path) -> bool {
    if path.ends_with(command) && path.is_executable() {
        println!("{}", path.display());
        return true;
    }
    false
}

pub fn check_type(command: &str, built_in_commands: &HashSet<&str>) -> io::Result<()> {
    let is_built_in = built_in_commands.contains(&command);
    if is_built_in {
        println!("{command} is a shell builtin");
        return Ok(());
    }

    let paths = env!("PATH");

    for path_buf in std::env::split_paths(&paths) {
        let p = path_buf.as_path();
        if command == "my_exe" {
            println!("Path: {}", p.display());
        }
        if p.is_file() && is_command_at_path(command, &p) {
            return Ok(());
        }

        if p.is_dir() {
            for entry in fs::read_dir(p)? {
                let entry = entry?;
                let path = entry.path();
                if command == "my_exe" {
                    println!("\t\t SubPath: {}", path.display());
                }
                
                if is_command_at_path(command, &path) {
                    return Ok(());
                }
            }
        }
    }
    println!("{command}: not found");
    Ok(())
}