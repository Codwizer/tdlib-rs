// Copyright 2024 - developers of the `tdlib-rs` project.

// MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module gathers all the code generation submodules and coordinates
//! them, feeding them the right data.
mod enums;
mod functions;
mod metadata;
mod rustifier;
mod types;

use std::io::{self, Write};
use parser_tdlib_tl::tl::{Definition, Type};

/// Don't generate types for definitions of this type,
/// since they are "core" types and treated differently.
const SPECIAL_CASED_TYPES: [&str; 6] = ["Bool", "Bytes", "Int32", "Int53", "Int64", "Ok"];

fn ignore_type(ty: &Type) -> bool {
    SPECIAL_CASED_TYPES.iter().any(|&x| x == ty.name)
}

pub fn generate_rust_code(
    file: &mut impl Write,
    definitions: &[Definition],
    gen_bots_only_api: bool,
) -> io::Result<()> {
    write!(
        file,
        "\
         // Copyright 2024 - developers of the `tdlib-rs` project.\n\
         //\n\
         // MIT license\n\
         // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your\n\
         // option. This file may not be copied, modified, or distributed\n\
         // except according to those terms.\n\
         "
    )?;

    let metadata = metadata::Metadata::new(definitions);
    types::write_types_mod(file, definitions, &metadata, gen_bots_only_api)?;
    enums::write_enums_mod(file, definitions, &metadata, gen_bots_only_api)?;
    functions::write_functions_mod(file, definitions, &metadata, gen_bots_only_api)?;

    Ok(())
}
