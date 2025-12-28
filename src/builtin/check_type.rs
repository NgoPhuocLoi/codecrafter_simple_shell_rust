
use std::collections::HashSet;
use std::fs;
use std::io;
use is_executable::IsExecutable;

pub fn check_type(command: &str, built_in_commands: &HashSet<&str>) -> io::Result<()> {
    let is_built_in = built_in_commands.contains(&command);
    if is_built_in {
        println!("{command} is a shell builtin");
        return Ok(());
    }

    let paths = env!("PATH");

    for path_buf in std::env::split_paths(&paths) {
        let p = path_buf.as_path();
        if !p.is_dir() {
            return Ok(());
        }
        
        for entry in fs::read_dir(p)? {
            let entry = entry?;
            let path = entry.path();
            if path.ends_with(command) && path.is_executable() {
                println!("{}", path.display());
                return Ok(());
            }
        }
    }
    println!("{command}: not found");
    Ok(())
}