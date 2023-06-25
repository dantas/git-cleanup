use std::process::Command;
use std::process::ExitStatus;
use thiserror::Error;

pub fn execute<P, A, S>(path: &P, command: &S, args: &A) -> Result<String, ExecuteError>
where
    P: AsRef<std::path::Path>,
    A: AsRef<[S]> + IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut command = Command::new(command);

    command.current_dir(path).args(args.as_ref());

    let output = command.output()?;

    check_for_success(output.status)?;

    Ok(String::from_utf8(output.stdout)?)
}

fn check_for_success(status: ExitStatus) -> Result<(), ExecuteError> {
    if status.success() {
        return Ok(());
    }

    match status.code() {
        Some(code) => Err(ExecuteError::CommandErrorCode { code }),
        _ => Err(ExecuteError::CommandError),
    }
}

#[derive(Error, Debug)]
pub enum ExecuteError {
    #[error("Error executing command")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("Error parsing git command output")]
    Parse {
        #[from]
        source: std::string::FromUtf8Error,
    },

    #[error("Error executing command, status code {code}")]
    CommandErrorCode { code: i32 },

    #[error("Error executing command")]
    CommandError,
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! sequence_execute {
    ( $path:ident : ($command:literal, $($arg:expr),*) ) => {
        $crate::execute::execute(&$path, &$command, &[$($arg),*])?;
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
#[allow(unused_imports)]
pub(crate) use sequence_execute;
