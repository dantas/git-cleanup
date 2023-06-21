use crate::execute;
use crate::git::Branch;
use crate::git::Repository;

// --step
// --silent

pub fn clean<P>(path: P, repository: Repository, args: &[&str]) -> bool
where
    P: AsRef<std::path::Path>,
{
    let step_option = args.get(0).unwrap_or(&"") == &"--step";

    let local_branches = repository.branches.iter().filter(|branch| {
        if let Branch::Local { .. } = branch {
            true
        } else {
            false
        }
    });

    // for branch in local_branches {
    //     if step_option {
    //         println!("Deleting ")
    //     }
    //     execute::execute(dir, "git", ["branch", "-d", ]);
    // }

    println!("Clean {:?}", step_option);

    true
}
