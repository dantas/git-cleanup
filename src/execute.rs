use crate::git::GitError;
use crate::git;
use std::process::Command;
use std::process::ExitStatus;

pub fn execute<P, A, S>(dir: P, command: S, args: A) -> Result<String, GitError>
    where P : AsRef<std::path::Path>,
          A : AsRef<[S]> + IntoIterator<Item=S>,
          S : AsRef<std::ffi::OsStr> {
    let mut command = Command::new(command);

    command.current_dir(dir).args(args);

    let output = command.output()?;

    check_for_success(output.status)?;

    let stdout_as_string = String::from_utf8(output.stdout)?;

    Result::Ok(stdout_as_string)
}

fn check_for_success(status: ExitStatus) -> Result<(), GitError> {
    if status.success() {
        return Result::Ok(())
    }

    let error = match status.code() {
        Some(code) => {
            git::new_git_error_with_string!("Error executing command: {}", code)
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

#[cfg(test)]
macro_rules! sequence_execute {
    ( $path:ident : ($command:literal, $($arg:expr),*) ) => {
        let _ = $crate::execute::execute(&$path, &$command, [$(&$arg),*]);
    };

    ( $path:ident : $($command_and_args:tt),+ ) => {
        $(
            $crate::execute::sequence_execute! {
                $path:
                    $command_and_args
            }
        )+
    };

    ( $($path:ident : $($command_and_args:tt),+)+ ) => {
        $(
            $crate::execute::sequence_execute! {
                $path:
                    $($command_and_args),+
            }
        )+
    };
}

#[cfg(test)]
pub(super) use sequence_execute;