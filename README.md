# Git Cleanup

It is a small command line utilitary that deletes local branches that are gone from any origin.  

**Status**: In development

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
This is also why I deviated from the typical parser design when parsing git output.  
I did my best to minimize unnecessary memory allocations.  
The Vec<u8> outputted by the git binary is reused in the creation of String. This String is kept around so that Repository can have a bunch of &str pointing to it, preventing the allocation of many Strings.
The only issue left is that HashSet can suffer from [unnecessary realocations](https://github.com/dantas/git-cleanup/blob/4f745f673d74f7ee19532518f954786710352f6d/src/git/repository.rs#L16) because it is created without an appropriate capacity.