use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Arguments {
    pub options: HashSet<ProgramOption>,
    pub command: Option<ProgramCommand>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ProgramOption {
    Help,
    Path(PathBuf),
    FetchPrune,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramCommand {
    List(ListOption),
    Clean(CleanOption),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListOption {
    Help,
    All,
    Local,
    Tracked,
    Gone,
    Diverged,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CleanOption {
    Help,
    Step,
    Automatic,
}
