//! This crate provide routines for trait based searching and captureing strings for matches instead of regex crate.
//! This is developed as an alternative of regex crate.
//!
//! # Overview
//! The primary type in this crate is a `Rex`, and trait is `Matcher`.
//!
//! `Rex` implements `Matcher` trait.
//!
//! `Matcher` has two methods as follows:
//! - `Matcher::match_with` : reports whether a match matches with `target` and the length
//! - `Matcher::capture`: returns matched length and captured str slice.
//! - `Matcher::find`: find matches and returns the offset.
//!
//! # Example
//! ```
//! use rexify::Rex;
//! use rexify::Matcher;
//! use rexify::number::Number;
//! use rexify::any_char::AnyChar;
//! use rexify::repeats::Repeat1;
//!
//! let rex = Rex::new(vec![
//!     Box::new(Repeat1::new(Number::new())),
//!     Box::new(Repeat1::new(AnyChar::new())),
//! ]);
//! let text = "asdija123102abc";
//!
//! assert_eq!(rex.find(text), Some(6));
//! assert_eq!(rex.capture(&text[6 ..]), Some((9, vec!["1", "2", "3", "1", "0", "2", "a", "b", "c"])));
//! ````

pub mod any_char;
pub mod literal;
pub mod number;
pub mod alpha;
pub mod range;
pub mod repeats;
pub mod select;

pub use any_char::*;
pub use literal::*;
pub use number::*;
pub use alpha::*;
pub use range::*;
pub use repeats::*;
pub use select::*;

/// A trait for types that is used to match in a haystack.
/// # Example
/// ```
/// use rexify::Rex;
/// use rexify::Matcher;
/// use rexify::number::Number;
/// use rexify::any_char::AnyChar;
/// use rexify::repeats::Repeat1;
///
/// let rex = Rex::new(vec![
///     Box::new(Repeat1::new(Number::new())),
///     Box::new(Repeat1::new(AnyChar::new())),
/// ]);
/// let text = "asdija123102abc";
///
/// assert_eq!(rex.find(text), Some(6));
/// assert_eq!(rex.capture(&text[6 ..]), Some((9, vec!["1", "2", "3", "1", "0", "2", "a", "b", "c"])));
/// ````
pub trait Matcher {
    /// If `self` is matches with `target` from first, returns the length of matches.
    fn match_with(&self, target: &str) -> Option<usize>;
    /// If `self` is matches with `target` from first, returns the length of matches and captures.
    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)>;

    /// Search matches in `target` and returns first found index.
    fn find(&self, target: &str) -> Option<usize> {
        let mut index = 0;
        for c in target.chars() {
            if let Some(_) = self.match_with(&target[index..]) {
                return Some(index);
            }
            index += c.len_utf8();
        }

        None
    }
}

/// Search matches in a haystack. This is same to `Regex` type.
///
/// # Example
/// ```
/// use rexify::Rex;
/// use rexify::Matcher;
/// use rexify::number::Number;
/// use rexify::any_char::AnyChar;
/// use rexify::repeats::Repeat1;
///
/// let rex = Rex::new(vec![
///     Box::new(Repeat1::new(Number::new())),
///     Box::new(Repeat1::new(AnyChar::new())),
/// ]);
/// let text = "asdija123102abc";
///
/// assert_eq!(rex.find(text), Some(6));
/// assert_eq!(rex.capture(&text[6 ..]), Some((9, vec!["1", "2", "3", "1", "0", "2", "a", "b", "c"])));
/// ````
pub struct Rex {
    rule: Vec<Box<dyn Matcher>>,
}

impl Rex {
    /// Create a new `Rex` object.
    pub fn new(rule: Vec<Box<dyn Matcher>>) -> Self {
        Self { rule: rule }
    }
}

impl Matcher for Rex {
    fn match_with(&self, target: &str) -> Option<usize> {
        let mut index = 0;
        for rule in &self.rule {
            index += rule.match_with(&target[index..])?;
        }
        Some(index)
    }

    fn capture<'a>(&self, target: &'a str) -> Option<(usize, Vec<&'a str>)> {
        let mut captures = Vec::new();
        let mut index = 0;
        for rule in &self.rule {
            let (len, mut capture) = rule.capture(&target[index..])?;
            index += len;
            captures.append(&mut capture);
        }
        Some((index, captures))
    }
}
