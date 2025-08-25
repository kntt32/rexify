//! This module provides some repeat matchers.
//! - `Repeat`: like "*" in regex.
//! - `Repeat1`: like "+" in regex.
//! - `Optional`: like "?" in regex.

use super::Matcher;

/// `Repeat` is same to "*" in regex.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::repeats::Repeat;
/// use rexify::literal::Literal;
///
/// let rex = Rex::new(vec![
///     Box::new(Repeat::new(
///         Literal::new("Hello")
///     ))
/// ]);
/// 
/// assert_eq!(rex.find("123456 HelloHello, World!"), Some(0));
/// ```
pub struct Repeat<T: Matcher>(T);

/// `Repeat1` is same to "+" in regex.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::repeats::Repeat1;
/// use rexify::literal::Literal;
///
/// let rex = Rex::new(vec![
///     Box::new(Repeat1::new(
///         Literal::new("Hello")
///     ))
/// ]);
/// 
/// assert_eq!(rex.find("123456World!"), None);
/// ```
pub struct Repeat1<T: Matcher>(T);

/// `Optional` is same to "?" in regex.
/// # Example
/// ```
/// use rexify::Matcher;
/// use rexify::Rex;
/// use rexify::repeats::Optional;
/// use rexify::literal::Literal;
///
/// let rex = Rex::new(vec![
///     Box::new(Optional::new(
///         Literal::new("Hello")
///     ))
/// ]);
/// 
/// assert_eq!(rex.find("123456World!"), Some(0));
/// ```
pub struct Optional<T: Matcher>(T);

impl<T: Matcher> Repeat<T> {
    /// Create new `Repeat` object.
    pub fn new(repeat: T) -> Self {
        Self(repeat)
    }
}

impl<T: Matcher> Matcher for Repeat<T> {
    fn match_with(&self, target: &str) -> Option<usize> {
        let mut index = 0; 
        while let Some(len) = self.0.match_with(&target[index ..]) && len != 0 {
            index += len;
        }

        Some(index)
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        let mut index = 0;
        let mut capture = Vec::new();
        while let Some((len, mut cap)) = self.0.capture(&target[index ..]) && len != 0 {
            index += len;
            capture.append(&mut cap);
        }

        Some((index, capture))
    }
}

impl<T: Matcher> Repeat1<T> {
    /// Create new `Repeat1` object.
    pub fn new(repeat: T) -> Self {
        Self(repeat)
    }
}

impl<T: Matcher> Matcher for Repeat1<T> {
    fn match_with(&self, target: &str) -> Option<usize> {
        let mut index = 0; 
        while let Some(len) = self.0.match_with(&target[index ..]) && len != 0 {
            index += len;
        }

        if index != 0 {
            Some(index)
        }else {
            None
        }
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        let mut index = 0;
        let mut capture = Vec::new();
        while let Some((len, mut cap)) = self.0.capture(&target[index ..]) && len != 0 {
            index += len;
            capture.append(&mut cap);
        }

        if index != 0 {
            Some((index, capture))
        }else {
            None
        }
    }
}

impl<T: Matcher> Optional<T> {
    /// Create new `Optional` object.
    pub fn new(optional: T) -> Self {
        Self(optional)
    }
}

impl<T: Matcher> Matcher for Optional<T> {
    fn match_with(&self, target: &str) -> Option<usize> {
        self.0.match_with(target).or(Some(0))
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        self.0.capture(target).or_else(|| Some((0, Vec::new())))
    }
}
