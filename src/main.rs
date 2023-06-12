use git::GitError;

pub mod git;

fn main() -> Result<(), GitError> {
    // TODO: Setup integration tests to cover other possible repository layouts
 
    let repository = git::repository("/home/dantas/Documents/git-deleted-branches/test_dir/local")?;

    for branch in &repository.branches {
        if branch == &repository.current_branch {
            print!("* ")
        }
        
        match branch {
            git::Branch::Tracked { name, remote } => {
                println!("{} => [{}]", name, remote)
            }
            git::Branch::Local { name } => {
                println!("{}", name)
            }
            git:: Branch::Detached => println!("Detached")
        }
    }

    Result::Ok(())
}
