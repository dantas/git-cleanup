use std::env;

pub struct VecArgs(Vec<String>);

impl VecArgs {
    pub fn new() -> Self {
        VecArgs(Vec::from_iter(env::args().skip(1)))
    }

    pub fn vec_str(&self) -> Vec<&str> {
        Vec::from_iter(self.0.iter().map(|s| s.as_str()))
    }
}
