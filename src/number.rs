//! This module provides number matcher.
//! - `Number`: matches with any number literal.
//! - `Digit`: matches with any number charactor.

use super::Matcher;

/// Match with number literal.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::number::Number;
///
/// let rex = Rex::new(vec![
///     Box::new(Number::new())
/// ]);
///
/// assert_eq!(rex.find("aaaaa 123456 Hello, World!"), Some(6));
/// ```
pub struct Number();

/// Match with number digit.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::number::Digit;
///
/// let rex = Rex::new(vec![
///     Box::new(Digit::new())
/// ]);
///
/// assert_eq!(rex.find("aaaaa 1 Hello, World!"), Some(6));
/// ```
pub struct Digit();

impl Number {
    /// Create new `Number` object.
    pub fn new() -> Self {
        Number()
    }
}

impl Matcher for Number {
    fn match_with(&self, target: &str) -> Option<usize> {
        let mut chars = target.chars();
        let mut index = 0;
        while let Some(c) = chars.next()
            && c.is_ascii_digit()
        {
            index += c.len_utf8();
        }

        if index != 0 { Some(index) } else { None }
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        let len = self.match_with(target)?;
        Some((len, vec![&target[..len]]))
    }
}

impl Digit {
    /// Create new `Digit` object.
    pub fn new() -> Self {
        Digit()
    }
}

impl Matcher for Digit {
    fn match_with(&self, target: &str) -> Option<usize> {
        if target.starts_with(|c: char| c.is_ascii_digit()) {
            Some(1)
        } else {
            None
        }
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        let len = self.match_with(target)?;
        Some((len, vec![&target[..len]]))
    }
}
