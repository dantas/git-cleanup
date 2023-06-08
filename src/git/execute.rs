use crate::git::RepositoryError;
use std::process::Command;

pub fn git_command<P, A, S>(dir: P, args: A) -> Result<String, RepositoryError>
    where P : AsRef<std::path::Path>,
          A : AsRef<[S]> + IntoIterator<Item=S>,
          S : AsRef<std::ffi::OsStr> {
    let mut command = Command::new("git");

    command.current_dir(dir);
    command.args(args);

    let output = command.output()?;

    if !output.status.success() {
        let message = match output.status.code() {
            Some(code) => {
                format!("Error executing command: {}", code)
            }
            _ => {
                "Error executing command".to_owned()
            }
        };

        return Result::Err(
            RepositoryError::with_string(message)
        )
    }

    let stdout = command.output()?.stdout;
    let output = String::from_utf8(stdout)?;

    Result::Ok(output)
}

// TODO: Check if is possible to use fn From