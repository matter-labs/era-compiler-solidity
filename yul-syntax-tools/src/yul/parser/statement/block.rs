//!
//! The source code block.
//!

use std::collections::HashSet;

use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::symbol::Symbol;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::dialect::Dialect;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::statement::assignment::Assignment;
use crate::yul::parser::statement::expression::Expression;
use crate::yul::parser::statement::Statement;

///
/// The Yul source code block.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(bound = "P: serde::de::DeserializeOwned")]
pub struct Block<P>
where
    P: Dialect,
{
    /// The location.
    pub location: Location,
    /// The block statements.
    pub statements: Vec<Statement<P>>,
}

impl<P> Block<P>
where
    P: Dialect,
{
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;

        let mut statements = Vec::new();

        let location = match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                location,
                ..
            } => location,
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["{"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };

        let mut remaining = None;

        loop {
            match crate::yul::parser::take_or_next(remaining.take(), lexer)? {
                token @ Token {
                    lexeme: Lexeme::Keyword(_),
                    ..
                } => {
                    let (statement, next) = Statement::parse(lexer, Some(token))?;
                    remaining = next;
                    statements.push(statement);
                }
                token @ Token {
                    lexeme: Lexeme::Literal(_),
                    ..
                } => {
                    statements
                        .push(Expression::parse(lexer, Some(token)).map(Statement::Expression)?);
                }
                token @ Token {
                    lexeme: Lexeme::Identifier(_),
                    ..
                } => match lexer.peek()? {
                    Token {
                        lexeme: Lexeme::Symbol(Symbol::Assignment),
                        ..
                    } => {
                        statements.push(
                            Assignment::parse(lexer, Some(token)).map(Statement::Assignment)?,
                        );
                    }
                    Token {
                        lexeme: Lexeme::Symbol(Symbol::Comma),
                        ..
                    } => {
                        statements.push(
                            Assignment::parse(lexer, Some(token)).map(Statement::Assignment)?,
                        );
                    }
                    _ => {
                        statements.push(
                            Expression::parse(lexer, Some(token)).map(Statement::Expression)?,
                        );
                    }
                },
                token @ Token {
                    lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                    ..
                } => statements.push(Block::parse(lexer, Some(token)).map(Statement::Block)?),
                Token {
                    lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                    ..
                } => break,
                token => {
                    return Err(ParserError::InvalidToken {
                        location: token.location,
                        expected: vec!["{keyword}", "{expression}", "{identifier}", "{", "}"],
                        found: token.lexeme.to_string(),
                    }
                    .into());
                }
            }
        }

        Ok(Self {
            location,
            statements,
        })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        let mut libraries = HashSet::new();
        for statement in self.statements.iter() {
            libraries.extend(statement.get_missing_libraries());
        }
        libraries
    }
}

#[cfg(test)]
mod tests {
    use crate::yul::lexer::token::location::Location;
    use crate::yul::lexer::Lexer;
    use crate::yul::parser::dialect::DefaultDialect;
    use crate::yul::parser::error::Error;
    use crate::yul::parser::statement::object::Object;

    #[test]
    fn error_invalid_token_bracket_curly_left() {
        let input = r#"
object "Test" {
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                (
                    return(0, 0)
                }
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<DefaultDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidToken {
                location: Location::new(11, 17),
                expected: vec!["{keyword}", "{expression}", "{identifier}", "{", "}"],
                found: "(".to_owned(),
            }
            .into())
        );
    }

    #[test]
    fn error_invalid_token_statement() {
        let input = r#"
object "Test" {
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                :=
                return(0, 0)
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<DefaultDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidToken {
                location: Location::new(11, 17),
                expected: vec!["{keyword}", "{expression}", "{identifier}", "{", "}"],
                found: ":=".to_owned(),
            }
            .into())
        );
    }
}
