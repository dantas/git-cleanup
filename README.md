# Git Cleanup

It is a small command line utilitary that deletes local branches that are not present in any origin.  

# Available Commands

- **--help**: Show this help message
- **list**: List branches, grouping them by tracked and local
  - **--help**:     Show list command options
  - **--all**:       List all branches (default option)
  - **--tracked**:  List all tracked branches
  - **--local**:    List local branches

- **clean**: Delete local only branches
  - **--help**: Show clean command options
  - **--step**: Require user confirmation before deleting each branch

# Disclaimer

Be careful when using this program. It may contains bugs that can lead to data loss.  
Use it at your own peril.

# Developer notes

I'm not using the lib+bin approach. Integration tests are on their own module, inside the bin crate, behind a feature.  
Run cargo test with **-F integration** to execute integration tests. 