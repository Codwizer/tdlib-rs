// Copyright 2024 - developers of the `tdlib-rs` project.

// MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module contains all the different structures representing the
//! various terms of the [Type Language].
//!
//! [Type Language]: https://core.telegram.org/mtproto/TL
mod category;
mod definition;
mod parameter;
mod ty;

pub use category::Category;
pub use definition::Definition;
pub use parameter::Parameter;
pub use ty::Type;
