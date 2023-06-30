use regex::Regex;

pub(super) struct Line<'a>(Vec<&'a str>);

impl<'a> Line<'a> {
    pub(super) fn parse(line: &'a str) -> Self {
        let regex = Regex::new(r"(\[.*\])+|(\S)+").unwrap();

        let captures_iter = regex
            .captures_iter(line)
            .filter_map(|c| c.get(0))
            .map(|m| m.as_str());

        Line(Vec::from_iter(captures_iter))
    }

    pub(super) fn is_head(&self) -> bool {
        self.0.first().is_some_and(|s| *s == "*")
    }

    pub(super) fn components(&self) -> &[&'a str] {
        if self.is_head() {
            self.0[1..].as_ref()
        } else {
            self.0.as_slice()
        }
    }
}

impl<'a> std::fmt::Display for Line<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0.join(","))
    }
}

#[test]
fn is_head() {
    let sut = Line::parse("* Hello World");
    assert!(sut.is_head());
}

#[test]
fn split_words() {
    let sut = Line::parse("Hello World");
    let expected = ["Hello", "World"];
    assert_eq!(sut.components(), expected);
}

#[test]
fn split_components() {
    let line = Line::parse("Hello [inside square brackets] World");
    let expected = ["Hello", "[inside square brackets]", "World"];
    assert_eq!(line.components(), expected);
}

#[test]
fn empty_line() {
    let line = Line::parse("");
    let expected: [&str; 0] = [];
    assert_eq!(line.components(), expected);
}
