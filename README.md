# 🍵 Rexify
**Trait-driven matcher library for Rust - expressive, composable, and extendable!**

## ⚡ What is Rexify?
`Rexify` enables Rustaceans to define matchers using a trait based approach, offering a more **type-safe**, **descriptive**, and **extensible** alternative to traditional regex. It empowers you to create text parsers with ease.

## 🔎 Highlights
- **Modular & Extensible**: Compose custom patterns by implementing the `Matcher` trait.
- **Rust-Native Design**: Leverages trait objects, ownership and lifetime for safe and efficient matching.

## 📦 Intallation
Add `rexify` to your `Cargo.toml`:
```toml
[dependencies]
rexify = "0.2.0"
```

## 🔧 Examples
```
use rexify::Rex;
use rexify::Matcher;
use rexify::number::Number;
//! use rexify::any_char::AnyChar;
//! use rexify::repeats::Repeat1;
//!
//! let rex = Rex::new(vec![
//!     Box::new(Number::new()),
//!     Box::new(Repeat1::new(AnyChar::new())),
//! ]);
//! let text = "asdija123102abc";
//!
//! assert_eq!(rex.find(text), Some(6));
//! assert_eq!(rex.capture(&text[6 ..]), Some((9, vec!["123102", "a", "b", "c"])));
//! ````

## ⚖️ Lisence
MIT Lisence

