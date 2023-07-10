# Git Cleanup

It is a small command line utilitary that deletes local branches that are gone from any origin.  

**Status**: Completed

# Help

**Options**:  
  - **--help**: Print help  
  - **--path <PATH>**: Execute operations in another path. Default is current directory.  
  - **--fetch-prune**: Execute *git fetch --prune* before executing specified command  

**Commands**:
  - **list**: List branches
    - **--help**: Print list options
    - **--gone**:     List tracking branches that are gone from origin (default option)
    - **--diverged**: List tracking branches that diverged from origin
    - **--all**:  List all local and tracked branches
    - **--tracked**:  List all tracked branches
    - **--local**:    List local branches
  - **clean**: Delete local branches that are gone from origin
    - **--help**:      Print clean options
    - **--step**: Ask for user confirmation before deleting each branch (default option)
    - **--automatic**: Delete branches without asking for user confirmation
</code>

# Disclaimer

Be careful when using this program. It may contain bugs that can lead to data loss.  
Use it at your own peril.

# Developer notes

Tests that execute external binaries require the feature **testbin**  
Because this is my first Rust project, I'm choosing to use the minimum amout of crates so that I can code more, not less.  
I also deviated from the typical parser design (when parsing git output) because I wanted to use some specific Rust features.  