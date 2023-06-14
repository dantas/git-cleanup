use std::env;

pub struct VecArgs {
    args: Vec<String>
}

impl VecArgs {
    pub fn new() -> Self {
        VecArgs {
            args: env::args().skip(1).collect::<Vec<String>>(),
        }
    }

    pub fn as_vec_str(&self) -> Vec<&str> {
        self.args.iter().map(|s| s.as_str()).collect::<Vec<&str>>()
    }
}
