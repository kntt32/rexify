use super::Matcher;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Space();

impl Space {
    pub fn new() -> Self {
        Self()
    }
}

impl Matcher for Space {
    fn match_with(&self, target: &str) -> Option<usize> {
        let c = target.chars().next()?;
        if let Some(c) = target.chars().next && c.is_whitespace() {
            Some(c.len_utf8())
        }else {
            None
        }
    }

    fn capture(&self, target: &str) -> Option<(usize, Vec<&'a str>)> {
        let len = self.match_with(target)?;
        Some((len, Vec::new()))
    }
}

