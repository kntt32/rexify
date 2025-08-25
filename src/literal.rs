//! This module provides string literal matcher.
//! - `Literal`: matches with string literal.

use super::Matcher;

/// Match with string literal.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::literal::Literal;
///
/// let rex = Rex::new(vec![
///     Box::new(Literal::new("Hello"))
/// ]);
///
/// assert_eq!(rex.find("123456 Hello, World!"), Some(7));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Literal<'a>(&'a str);

impl<'a> Literal<'a> {
    /// Create new `Literal` object.
    pub fn new(literal: &'a str) -> Self {
        Self(literal)
    }
}

impl Matcher for Literal<'_> {
    fn match_with(&self, target: &str) -> Option<usize> {
        if target.starts_with(self.0) {
            Some(self.0.len())
        } else {
            None
        }
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        let len = self.match_with(target)?;
        Some((len, vec![&target[..len]]))
    }
}
