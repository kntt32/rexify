//! This module provides number matcher.
//! - `Number`: matches with any number charactor.

use super::Matcher;

/// Match with number charactor.
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

impl Number {
    pub fn new() -> Self {
        Number()
    }
}

impl Matcher for Number {
    fn match_with(&self, target: &str) -> Option<usize> {
        if target.starts_with(|c: char| c.is_ascii_digit()) {
            Some(1)
        }else {
            None
        }
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        let len = self.match_with(target)?;
        Some((len, vec![&target[..len]]))
    }
}
