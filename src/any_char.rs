//! This module provides any char matcher like "." in regex.
//! - `AnyChar`: matches with any charactor.

use super::Matcher;

/// Match with any one charactor.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::any_char::AnyChar;
///
/// let rex = Rex::new(vec![
///     Box::new(AnyChar::new())
/// ]);
///
/// assert_eq!(rex.find("a"), Some(0));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnyChar();

impl AnyChar {
    pub fn new() -> Self {
        AnyChar()
    }
}

impl Matcher for AnyChar {
    fn match_with(&self, target: &str) -> Option<usize> {
        Some(target.chars().next()?.len_utf8())
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        let len = self.match_with(target)?;
        Some((len, vec![&target[..len]]))
    }
}
