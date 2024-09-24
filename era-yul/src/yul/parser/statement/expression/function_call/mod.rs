//!
//! The function call subexpression.
//!

pub mod name;

use std::collections::HashSet;

use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::literal::Literal as LexicalLiteral;
use crate::yul::lexer::token::lexeme::symbol::Symbol;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::statement::expression::literal::Literal;
use crate::yul::parser::statement::expression::Expression;

use self::name::Name;

///
/// The Yul function call subexpression.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    /// The location.
    pub location: Location,
    /// The function name.
    pub name: Name,
    /// The function arguments expression list.
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;

        let (location, name) = match token {
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
                ..
            } => (location, Name::from(identifier.inner.as_str())),
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["{identifier}"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };

        let mut arguments = Vec::new();
        loop {
            let argument = match lexer.next()? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                    ..
                } => break,
                token => Expression::parse(lexer, Some(token))?,
            };

            arguments.push(argument);

            match lexer.peek()? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::Comma),
                    ..
                } => {
                    lexer.next()?;
                    continue;
                }
                Token {
                    lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                    ..
                } => {
                    lexer.next()?;
                    break;
                }
                _ => break,
            }
        }

        Ok(Self {
            location,
            name,
            arguments,
        })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        let mut libraries = HashSet::new();

        if let Name::LinkerSymbol = self.name {
            let _argument = self.arguments.first().expect("Always exists");
            if let Expression::Literal(Literal {
                inner: LexicalLiteral::String(library_path),
                ..
            }) = self.arguments.first().expect("Always exists")
            {
                libraries.insert(library_path.to_string());
            }
            return libraries;
        }

        for argument in self.arguments.iter() {
            libraries.extend(argument.get_missing_libraries());
        }
        libraries
    }
}
