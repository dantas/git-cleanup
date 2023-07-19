use std::process::{Command, ExitStatus};
use thiserror::Error;

pub fn execute<P>(path: &P, command: &str, args: &[&str]) -> Result<String, ExecuteError>
where
    P: AsRef<std::path::Path>,
{
    let mut command = Command::new(command);

    command.current_dir(path).args(args);

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

#[test]
#[cfg(feature = "testbin")]
fn success_execution() {
    let some_dir = std::env::current_dir().unwrap();
    let sut = execute(&some_dir, "echo", &["Hello world\nMultiple lines"]).unwrap();
    let expected = "Hello world\nMultiple lines\n";
    assert_eq!(sut, expected);
}

#[test]
#[cfg(feature = "testbin")]
fn error_execution() {
    let some_dir = std::env::current_dir().unwrap();
    execute(&some_dir, "git", &["something"]).expect_err("Execute should've failed");
}

#[cfg(test)]
#[allow(unused_macros)]
// Limitation: Command must receive an argument, otherwise the compiler will hit recursion limit
macro_rules! sequence_execute {
    ( $path:ident : ($command:literal, $($arg:expr),*) ) => {
        // SAFETY: Since this is only used for tests, it is OK to panic if an error occurred
        let _ = $crate::execute::execute(&$path, $command, &[$($arg),*]).unwrap();
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

#[test]
#[cfg(feature = "testbin")]
fn test_sequence_execute() {
    use crate::test_support::TempDir;

    let root = TempDir::new().unwrap();
    let subdir_one = root.join("subdir_one");
    let subdir_two = root.join("subdir_two");

    sequence_execute! {
        root:
            ("touch", "root_file"),
            ("mkdir", "subdir_one"),
            ("mkdir", "subdir_two")

        subdir_one:
            ("touch", "subdir_one_file1"),
            ("touch", "subdir_one_file2")

        subdir_two:
            ("touch", "subdir_two_file1"),
            ("touch", "subdir_two_file2")
    }

    let paths = [
        root.file_path("root_file"),
        subdir_one.file_path("subdir_one_file1"),
        subdir_one.file_path("subdir_one_file2"),
        subdir_two.file_path("subdir_two_file1"),
        subdir_two.file_path("subdir_two_file2"),
    ];

    for path in paths {
        match path.try_exists() {
            Ok(exists) => {
                if !exists {
                    panic!(
                        "Error executing file_sequence!. File ({}) does not exist",
                        path.to_string_lossy()
                    );
                }
            }
            Err(error) => panic!(
                "Error while checking file ({}) exists: {error}",
                path.to_string_lossy()
            ),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use sequence_execute;
