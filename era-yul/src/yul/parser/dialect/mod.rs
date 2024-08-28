//!
//! Describes a pragmatic, target-specific part of the parser.
//!

use std::collections::BTreeSet;
use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::yul::error::Error;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::Lexer;

use super::identifier::Identifier;

///
/// Describes a pragmatic, target-specific part of the parser.
///
pub trait Dialect: for<'de> Deserialize<'de> + Serialize + Eq + PartialEq + Clone + Debug {
    /// Type of function attributes parsed from their identifiers.
    type FunctionAttribute: for<'de> Deserialize<'de>
        + Debug
        + Clone
        + Eq
        + PartialEq
        + Serialize
        + Ord;

    ///
    /// Extractor for the function attributes.
    ///
    fn extract_attributes(
        identifier: &Identifier,
        lexer: &mut Lexer,
    ) -> Result<BTreeSet<Self::FunctionAttribute>, Error>;

    ///
    /// Check the dialect-specific function invariants and potentially modify
    /// their arguments list.
    ///
    fn sanitize_function(
        identifier: &Identifier,
        arguments: &mut Vec<Identifier>,
        location: Location,
        lexer: &mut Lexer,
    ) -> Result<(), Error>;
}

///
/// The root dialect without target-dependent features.
///
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Eq, PartialEq)]
pub struct DefaultDialect {}

impl Dialect for DefaultDialect {
    type FunctionAttribute = u32;

    fn extract_attributes(
        _identifier: &Identifier,
        _lexer: &mut Lexer,
    ) -> Result<BTreeSet<Self::FunctionAttribute>, Error> {
        Ok(BTreeSet::new())
    }

    fn sanitize_function(
        _identifier: &Identifier,
        _arguments: &mut Vec<Identifier>,
        _location: Location,
        _lexer: &mut Lexer,
    ) -> Result<(), Error> {
        Ok(())
    }
}
