// Copyright 2024 - developers of the `tdlib-rs` project.

// MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The category to which a definition belongs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    /// The default category, a definition represents a type.
    Types,

    /// A definition represents a callable function.
    Functions,
}
