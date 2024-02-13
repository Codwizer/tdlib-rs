// Copyright 2024 - developers of the `tdlib-rs` project.

// MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library provides a public interface to parse [Type Language]
//! definitions.
//!
//! It exports a single public method, [`parse_tl_file`] to parse entire
//! `.tl` files and yield the definitions it contains. This method will
//! yield [`Definition`]s containing all the information you would possibly
//! need to later use somewhere else (for example, to generate code).
//!
//! [Type Language]: https://core.telegram.org/mtproto/TL
//! [`parse_tl_file`]: fn.parse_tl_file.html
//! [`Definition`]: tl/struct.Definition.html
pub mod errors;
pub mod tl;
mod tl_iterator;

use errors::ParseError;
use tl::Definition;
use tl_iterator::TlIterator;

/// Parses a file full of [Type Language] definitions.
///
/// # Examples
///
/// ```no_run
/// use std::fs::File;
/// use std::io::{self, Read};
/// use tdlib_tl_parser::parse_tl_file;
///
/// fn main() -> std::io::Result<()> {
///     let mut file = File::open("api.tl")?;
///     let mut contents = String::new();
///     file.read_to_string(&mut contents)?;
///
///     for definition in parse_tl_file(contents) {
///         dbg!(definition);
///     }
///
///     Ok(())
/// }
/// ```
///
/// [Type Language]: https://core.telegram.org/mtproto/TL
pub fn parse_tl_file(contents: String) -> impl Iterator<Item = Result<Definition, ParseError>> {
    TlIterator::new(contents)
}
