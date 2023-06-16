use std::env;

pub struct VecArgs(Vec<String>);

impl VecArgs {
    pub fn new() -> Self {
        VecArgs(env::args().skip(1).collect::<Vec<String>>())
    }

    pub fn as_vec_str(&self) -> Vec<&str> {
        self.0.iter().map(|s| s.as_str()).collect::<Vec<&str>>()
    }
}
