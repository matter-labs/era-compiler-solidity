//!
//! Describes a pragmatic, target-specific part of the parser.
//!

pub mod llvm;

use std::collections::BTreeSet;
use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::yul::error::Error;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::Lexer;

use super::identifier::Identifier;

/// Describes a pragmatic, target-specific part of the parser.
pub trait Dialect: for<'de> Deserialize<'de> + Serialize + Eq + PartialEq + Clone + Debug {
    /// Type of function attributes parsed from their identifiers.
    type FunctionAttribute: for<'de> Deserialize<'de>
        + Debug
        + Clone
        + Eq
        + PartialEq
        + Serialize
        + Ord;

    /// Extractor for the function attributes.
    fn extract_attributes(
        identifier: &Identifier,
        lexer: &mut Lexer,
    ) -> Result<BTreeSet<Self::FunctionAttribute>, Error>;

    /// Check the dialect-specific function invariants and potentially modify
    /// their arguments list.
    fn sanitize_function(
        identifier: &Identifier,
        arguments: &mut Vec<Identifier>,
        location: Location,
        lexer: &mut Lexer,
    ) -> Result<(), Error>;
}
