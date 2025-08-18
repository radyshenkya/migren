use log::debug;
use regex::Regex;

#[derive(Debug)]
pub enum SqlDirective {
    Split,
}

impl SqlDirective {
    pub fn directive_regex(&self) -> Regex {
        match self {
            SqlDirective::Split => Regex::new(r"--.*migren:split.*").unwrap(),
        }
    }

    pub fn match_self_str(self, line: &str) -> Option<Self> {
        let reg = self.directive_regex();

        if reg.is_match(line.trim()) {
            return Some(self);
        }

        None
    }

    pub fn match_str(line: &str) -> Option<Self> {
        [SqlDirective::Split.match_self_str(line)]
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .next()
    }
}
