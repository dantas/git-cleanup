// Since everything is being made public inside the git module, I'm for now forced to use a longer name
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum RemoteBranchStatus {
    Gone,
    Diverged,
    Synchronized,
}

impl RemoteBranchStatus {
    pub(super) fn parse(string: &str) -> Self {
        // I'm taking some shortcuts so I'm using contains because it is easier to code
        if string.contains("ahead") || string.contains("behind") {
            return RemoteBranchStatus::Diverged;
        }

        if string.contains("gone") {
            return RemoteBranchStatus::Gone;
        }

        RemoteBranchStatus::Synchronized
    }
}

#[test]
fn ahead() {
    let sut = RemoteBranchStatus::parse("ahead 1");
    let expected = RemoteBranchStatus::Diverged;
    assert_eq!(sut, expected);
}

#[test]
fn behind() {
    let sut = RemoteBranchStatus::parse("behind 1");
    let expected = RemoteBranchStatus::Diverged;
    assert_eq!(sut, expected);
}

#[test]
fn ahead_and_behind() {
    let sut = RemoteBranchStatus::parse("ahead 1, behind 1");
    let expected = RemoteBranchStatus::Diverged;
    assert_eq!(sut, expected);
}

#[test]
fn gone() {
    let sut = RemoteBranchStatus::parse("gone");
    let expected = RemoteBranchStatus::Gone;
    assert_eq!(sut, expected);
}

#[test]
fn synchronized() {
    let sut = RemoteBranchStatus::parse("");
    let expected = RemoteBranchStatus::Synchronized;
    assert_eq!(sut, expected);
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! remote_status {
    ( gone ) => {
        $crate::git::RemoteBranchStatus::Gone
    };
    ( diverged ) => {
        $crate::git::RemoteBranchStatus::Diverged
    };
    ( synchronized ) => {
        $crate::git::RemoteBranchStatus::Synchronized
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use remote_status;
