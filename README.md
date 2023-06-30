# Git Cleanup

It is a small command line utilitary that deletes local branches that are gone from any origin.  

**Status**: In development

# Available Commands

- **--help**: Show this help message
- **list**: List branches
  - **--help**:    Show list command options
  - **--gone**:   List tracking branches that are gone from origin (default option)
  - **--diverged**: List tracking branches that diverged from origin
  - **--all**:      List all local and tracked branches
  - **--tracked**:  List all tracked branches
  - **--local**:   List local branches

- **clean**: Delete local branches that are gone from origin
  - **--help**:     Show clean command options
  - **--step**:     Ask for user confirmation before deleting each branch (default option)
  - **--automatic**: Delete branches without asking for user output

# Disclaimer

Be careful when using this program. It may contain bugs that can lead to data loss.  
Use it at your own peril.

# Developer notes

I'm not using the lib+bin approach. Integration tests are on their own module, inside the bin crate, behind a feature.  
Run cargo test with **-F integration** to execute integration tests. 

# Design Decisions

Since this is my first Rust project, I'm choosing to use the minimum amout of crates, forcing me to implement what I need, therefore creating more opportunities to practice the language.