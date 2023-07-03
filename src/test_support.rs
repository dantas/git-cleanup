#![cfg(all(test, feature = "testbin"))]

use std::env;
use std::fs;
use std::path::PathBuf;

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let random_dir_name = rand::random::<u32>().to_string();
        let path = env::temp_dir().join(random_dir_name);
        fs::create_dir(path.clone())?;
        Ok(TempDir { path })
    }

    pub fn join<P: AsRef<std::path::Path>>(&self, path: P) -> TempDir {
        TempDir {
            path: self.path.join(path.as_ref()),
        }
    }

    pub fn file_path<P: AsRef<std::path::Path>>(&self, path: P) -> PathBuf {
        self.path.join(path.as_ref())
    }
}

impl AsRef<std::path::Path> for TempDir {
    fn as_ref(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir(self);
    }
}
