use is_executable::IsExecutable;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Custom error type for executable search operations
#[derive(Debug)]
pub enum ExecutableSearchError {
    DirectoryReadError(String),
    PathEntryError(String),
}

impl fmt::Display for ExecutableSearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutableSearchError::DirectoryReadError(msg) => {
                write!(f, "Directory read error: {}", msg)
            }
            ExecutableSearchError::PathEntryError(msg) => write!(f, "Path entry error: {}", msg),
        }
    }
}

impl std::error::Error for ExecutableSearchError {}

/// Checks if a path exactly matches the command name and is executable
fn is_exact_match(command: &str, path: &Path) -> bool {
    path.ends_with(command) && path.is_executable()
}

/// Checks if a path's filename starts with the command prefix and is executable
fn is_prefix_match(command: &str, path: &Path) -> bool {
    if !path.is_executable() {
        return false;
    }

    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with(command))
        .unwrap_or(false)
}

/// Generic function to search for executables in PATH using a custom matcher
fn find_in_path<F>(command: &str, matcher: F) -> Result<Option<PathBuf>, ExecutableSearchError>
where
    F: Fn(&str, &Path) -> bool,
{
    let paths = match env::var_os("PATH") {
        Some(paths) => paths,
        None => return Ok(None),
    };

    for path_buf in std::env::split_paths(&paths) {
        let p = path_buf.as_path();

        if p.is_file() && matcher(command, p) {
            return Ok(Some(path_buf));
        }

        if p.is_dir() {
            let entries = fs::read_dir(p).map_err(|e| {
                ExecutableSearchError::DirectoryReadError(format!("{}: {}", p.display(), e))
            })?;

            for entry in entries {
                let entry = entry.map_err(|e| {
                    ExecutableSearchError::PathEntryError(format!("{}: {}", p.display(), e))
                })?;
                let path = entry.path();

                if matcher(command, &path) {
                    return Ok(Some(path));
                }
            }
        }
    }

    Ok(None)
}

/// Finds an executable in PATH that exactly matches the command name
pub fn find_executable_path(command: &str) -> Result<Option<PathBuf>, ExecutableSearchError> {
    find_in_path(command, is_exact_match)
}

/// Finds an executable in PATH whose filename starts with the command prefix
pub fn find_executable_path_like(command: &str) -> Result<Option<PathBuf>, ExecutableSearchError> {
    find_in_path(command, is_prefix_match)
}

/// Checks the type of a command (builtin or executable) and prints the result
pub fn check_type(command: &str, built_in_commands: &HashSet<&str>) {
    if built_in_commands.contains(command) {
        println!("{} is a shell builtin", command);
        return;
    }

    match find_executable_path(command) {
        Ok(Some(path)) => println!("{}", path.display()),
        Ok(None) => println!("{}: not found", command),
        Err(e) => eprintln!("Error searching for command: {}", e),
    }
}
