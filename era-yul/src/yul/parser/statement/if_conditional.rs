//!
//! The if-conditional statement.
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
/// The Yul if-conditional statement.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(bound = "P: serde::de::DeserializeOwned")]
pub struct IfConditional<P>
where
    P: Dialect,
{
    /// The location.
    pub location: Location,
    /// The condition expression.
    pub condition: Expression,
    /// The conditional block.
    pub block: Block<P>,
}

impl<P> IfConditional<P>
where
    P: Dialect,
{
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;
        let location = token.location;

        let condition = Expression::parse(lexer, Some(token))?;

        let block = Block::parse(lexer, None)?;

        Ok(Self {
            location,
            condition,
            block,
        })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        let mut libraries = self.condition.get_missing_libraries();
        libraries.extend(self.block.get_missing_libraries());
        libraries
    }
}
