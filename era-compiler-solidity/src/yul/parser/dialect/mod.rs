//!
//! Describes a pragmatic, target-specific part of the parser.
//!

pub mod era;

use std::collections::BTreeSet;
use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use era_yul::yul::error::Error;
use era_yul::yul::lexer::token::location::Location;
use era_yul::yul::lexer::Lexer;

use era_yul::yul::parser::identifier::Identifier;

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
