//! This module provides range matcher.
//! - `Select`: like "A-Z" in regex.

use super::Matcher;

/// Match with range of charactor.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::range::Range;
///
/// let rex = Rex::new(vec![
///     Box::new(Range::new('a', 'e'))
/// ]);
///
/// assert_eq!(rex.find("123456 Hello, World!"), Some(8));
/// ```
pub struct Range {
    start: char,
    end: char,
}

impl Range {
    /// Create new `Range` object.
    pub fn new(start: char, end: char) -> Self {
        Self {
            start: start,
            end: end,
        }
    }
}

impl Matcher for Range {
    fn match_with(&self, target: &str) -> Option<usize> {
        let c = target.chars().next()?;
        if self.start <= c && c <= self.end {
            Some(c.len_utf8())
        } else {
            None
        }
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        let len = self.match_with(target)?;
        Some((len, vec![&target[..len]]))
    }
}
