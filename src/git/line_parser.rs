use regex::Regex;
use std::iter::Peekable;
use std::sync::OnceLock;

static COMPONENTS_REGEX: OnceLock<Regex> = OnceLock::new();

pub(super) trait LineParser<'a> {
    fn consume_if_head(&mut self) -> bool;
    fn consume_if_detached(&mut self) -> bool;
    fn consume_components(&mut self) -> Option<LineComponents<'a>>;
}

#[derive(PartialEq, Eq, Debug)]
pub(super) struct LineComponents<'a> {
    pub branch_name: &'a str,
    pub maybe_origin_branch: &'a str,
}

pub(super) fn new_line_parser(line: &str) -> impl LineParser {
    let regex = COMPONENTS_REGEX.get_or_init(|| Regex::new(r"(\[.*\])+|(\S)+").unwrap());
    let find_iter: regex::Matches<'_, '_> = regex.find_iter(line);
    let iter = find_iter.map(|m| m.as_str());
    LineParserStruct(iter.peekable())
}

struct LineParserStruct<'a, I>(Peekable<I>)
where
    I: Iterator<Item = &'a str>;

impl<'a, I> LineParser<'a> for LineParserStruct<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    fn consume_if_head(&mut self) -> bool {
        self.consume_if_token("*")
    }

    fn consume_if_detached(&mut self) -> bool {
        self.consume_if_token("(HEAD")
    }

    fn consume_components(&mut self) -> Option<LineComponents<'a>> {
        let branch_name = self.0.next()?;
        let _ = self.0.next()?;
        let maybe_origin_branch = self.0.next()?;

        Some(LineComponents {
            branch_name,
            maybe_origin_branch,
        })
    }
}

impl<'a, I> LineParserStruct<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    fn consume_if_token(&mut self, expected_token: &'static str) -> bool {
        match self.0.peek() {
            Some(peeked_token) if *peeked_token == expected_token => {
                let _ = self.0.next();
                true
            }
            _ => false,
        }
    }
}

// impl<'a> std::fmt::Display for OldLine<'a> {
//     fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(formatter, "{}", self.0.join(","))
//     }
// }

#[test]
fn is_head() {
    let mut sut = new_line_parser("* main 73b4084 commit message");
    assert!(sut.consume_if_head());
}

#[test]
fn is_detached() {
    let mut sut = new_line_parser("(HEAD and other stuff");
    assert!(sut.consume_if_detached());
}

#[test]
fn split_components() {
    let mut sut = new_line_parser("develop 73b4084 [origin/develop] commit message");

    let components = sut.consume_components().unwrap();

    let expected = LineComponents {
        branch_name: "develop",
        maybe_origin_branch: "[origin/develop]",
    };

    assert_eq!(components, expected);
}

#[test]
fn empty_line() {
    let mut parser = new_line_parser("");
    assert_eq!(parser.consume_components(), None);
}
