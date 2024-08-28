//!
//! The for-loop statement.
//!

use std::collections::HashSet;

use crate::yul::error::Error;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::dialect::Dialect;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::expression::Expression;

///
/// The Yul for-loop statement.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(bound = "P: serde::de::DeserializeOwned")]
pub struct ForLoop<P>
where
    P: Dialect,
{
    /// The location.
    pub location: Location,
    /// The index variables initialization block.
    pub initializer: Block<P>,
    /// The continue condition block.
    pub condition: Expression,
    /// The index variables mutating block.
    pub finalizer: Block<P>,
    /// The loop body.
    pub body: Block<P>,
}

impl<P> ForLoop<P>
where
    P: Dialect,
{
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;
        let location = token.location;

        let initializer = Block::parse(lexer, Some(token))?;

        let condition = Expression::parse(lexer, None)?;

        let finalizer = Block::parse(lexer, None)?;

        let body = Block::parse(lexer, None)?;

        Ok(Self {
            location,
            initializer,
            condition,
            finalizer,
            body,
        })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        let mut libraries = self.initializer.get_missing_libraries();
        libraries.extend(self.condition.get_missing_libraries());
        libraries.extend(self.finalizer.get_missing_libraries());
        libraries.extend(self.body.get_missing_libraries());
        libraries
    }
}
