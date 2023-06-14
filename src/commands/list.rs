use crate::git::{Repository, Branch};

pub fn list(repository: &Repository) {
    for branch in &repository.branches {
        if branch == &repository.current_branch {
            print!("* ")
        }
        
        match branch {
            Branch::Tracked { name, remote } => {
                println!("{} => [{}]", name, remote)
            }
            Branch::Local { name } => {
                println!("{}", name)
            }
            Branch::Detached => println!("Detached")
        }
    }
}
