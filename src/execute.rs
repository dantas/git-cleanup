use crate::error::Error;
use crate::error;
use std::process::Command;
use std::process::ExitStatus;

pub fn execute<P, A, S>(path: P, command: S, args: A) -> Result<String, Error>
    where P : AsRef<std::path::Path>,
          A : AsRef<[S]> + IntoIterator<Item=S>,
          S : AsRef<std::ffi::OsStr> {
    let mut command = Command::new(command);

    command.current_dir(path).args(args);

    let output = command.output()?;

    check_for_success(output.status)?;

    let stdout_as_string = String::from_utf8(output.stdout)?;

    Ok(stdout_as_string)
}

fn check_for_success(status: ExitStatus) -> Result<(), Error> {
    if status.success() {
        return Ok(());
    }

    let error = match status.code() {
        Some(code) => {
            error::new_error_with_string!("Error executing command: {}", code)
        }
        _ => {
            Error::new_with_str("Error executing command")
        }
    };

    Err(error)
}           

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new_with_source(Box::new(error))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::new_with_source(Box::new(error))
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
pub(crate) use sequence_execute;