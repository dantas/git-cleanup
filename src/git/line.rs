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
