use crate::git::Branch;

#[derive(Debug, Clone, PartialEq)]
pub struct Repository {
    pub current_branch: Branch,
    pub branches: Vec<Branch>
}