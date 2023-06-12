use crate::git::GitError;
use std::process::Command;
use std::process::ExitStatus;

pub fn git_command<P, A, S>(dir: P, args: A) -> Result<String, GitError>
    where P : AsRef<std::path::Path>,
          A : AsRef<[S]> + IntoIterator<Item=S>,
          S : AsRef<std::ffi::OsStr> {
    let mut command = Command::new("git");

    command.current_dir(dir).args(args);

    let output = command.output()?;

    check_for_error(output.status)?;

    let stdout_as_string = String::from_utf8(output.stdout)?;

    Result::Ok(stdout_as_string)
}

fn check_for_error(status: ExitStatus) -> Result<(), GitError> {
    if status.success() {
        return Result::Ok(())
    }

    let error = match status.code() {
        Some(code) => {
            GitError::new_with_string(format!("Error executing command: {}", code))
        }
        _ => {
            GitError::new_with_str("Error executing command")
        }
    };

    Result::Err(error)
}

impl From<std::io::Error> for GitError {
    fn from(error: std::io::Error) -> Self {
        GitError::new_with_source(Box::new(error))
    }
}

impl From<std::string::FromUtf8Error> for GitError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        GitError::new_with_source(Box::new(error))
    }
}