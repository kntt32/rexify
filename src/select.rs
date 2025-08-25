//! This module provides selector matcher.
//! - `Select`: like "|" in regex.

use super::Matcher;

/// `Select` is same to "|" in regex.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::select::Select;
/// use rexify::literal::Literal;
///
/// let rex = Rex::new(vec![
///     Box::new(Select::new(
///         vec![
///             Box::new(Literal::new("Hello")),
///             Box::new(Literal::new("hello")),
///         ]
///     ))
/// ]);
///
/// assert_eq!(rex.find("123456 hello, Hello, World!"), Some(7));
/// ```

pub struct Select(Vec<Box<dyn Matcher>>);

impl Select {
    /// Create new `Repeat` object.
    pub fn new(rules: Vec<Box<dyn Matcher>>) -> Self {
        Select(rules)
    }
}

impl Matcher for Select {
    fn match_with(&self, target: &str) -> Option<usize> {
        for rule in &self.0 {
            if let Some(len) = rule.match_with(target) {
                return Some(len);
            }
        }

        None
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        for rule in &self.0 {
            if let Some(capture) = rule.capture(target) {
                return Some(capture);
            }
        }

        None
    }
}
