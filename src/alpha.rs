//! This module provides alphabet matcher.
//! - `Alpha`: matches with any alphabet charactor.

use super::Matcher;

/// Match with alphabet matcher.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::alpha::Alpha;
///
/// let rex = Rex::new(vec![
///     Box::new(Alpha::new())
/// ]);
///
/// assert_eq!(rex.find("123123a"), Some(6));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Alpha();

impl Alpha {
    pub fn new() -> Self {
        Self()
    }
}

impl Matcher for Alpha {
    fn match_with(&self, target: &str) -> Option<usize> {
        if target.starts_with(|c: char| c.is_ascii_alphabetic()) {
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
