//!
//! The expression statement.
//!

pub mod function_call;
pub mod literal;

use std::collections::HashSet;

use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::symbol::Symbol;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::identifier::Identifier;

use self::function_call::FunctionCall;
use self::literal::Literal;

///
/// The Yul expression statement.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum Expression {
    /// The function call subexpression.
    FunctionCall(FunctionCall),
    /// The identifier operand.
    Identifier(Identifier),
    /// The literal operand.
    Literal(Literal),
}

impl Expression {
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;

        let (location, identifier) = match token {
            Token {
                lexeme: Lexeme::Literal(_),
                ..
            } => return Ok(Self::Literal(Literal::parse(lexer, Some(token))?)),
            Token {
                location,
                lexeme: Lexeme::Identifier(identifier),
                ..
            } => (location, identifier),
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["{literal}", "{identifier}"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };
        let length = identifier.inner.len();

        match lexer.peek()? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            } => {
                lexer.next()?;
                Ok(Self::FunctionCall(FunctionCall::parse(
                    lexer,
                    Some(Token::new(location, Lexeme::Identifier(identifier), length)),
                )?))
            }
            _ => Ok(Self::Identifier(Identifier::new(
                location,
                identifier.inner,
            ))),
        }
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        match self {
            Self::FunctionCall(inner) => inner.get_missing_libraries(),
            Self::Identifier(_) => HashSet::new(),
            Self::Literal(_) => HashSet::new(),
        }
    }

    ///
    /// Returns the statement location.
    ///
    pub fn location(&self) -> Location {
        match self {
            Self::FunctionCall(inner) => inner.location,
            Self::Identifier(inner) => inner.location,
            Self::Literal(inner) => inner.location,
        }
    }
}
