# Git Cleanup

It is a small command line utilitary that deletes local branches that are not present in any origin.  

## Help

### Available Commands

- **--help**: Show this help message
- **list**: List branches, grouping them by tracked and local
  - **--help**:     Show list command help message
  - **--all**:       List all branches (default option)
  - **--tracked**:  List all tracked branches
  - **--local**:    List local branches

- **clean**: Delete local branches
  - **--help**: Show clean command help message
  - **--step**: Require user confirmation before deleting each branch

# Disclaimer

Be careful when using this program. It may contains bugs that can delete important work.  
Use it at your own peril.
