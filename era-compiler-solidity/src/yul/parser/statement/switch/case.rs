//!
//! The switch statement case.
//!

use std::collections::HashSet;

use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::expression::literal::Literal;

///
/// The Yul switch statement case.
///
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Case {
    /// The location.
    pub location: Location,
    /// The matched constant.
    pub literal: Literal,
    /// The case block.
    pub block: Block,
}

impl Case {
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;

        let (location, literal) = match token {
            token @ Token {
                lexeme: Lexeme::Literal(_),
                location,
                ..
            } => (location, Literal::parse(lexer, Some(token))?),
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["{literal}"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };

        let block = Block::parse(lexer, None)?;

        Ok(Self {
            location,
            literal,
            block,
        })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        self.block.get_missing_libraries()
    }
}

#[cfg(test)]
mod tests {
    use crate::yul::lexer::token::location::Location;
    use crate::yul::lexer::Lexer;
    use crate::yul::parser::error::Error;
    use crate::yul::parser::statement::object::Object;

    #[test]
    fn error_invalid_token_literal() {
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
                switch 42
                    case x {}
                    default {}
                }
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidToken {
                location: Location::new(12, 26),
                expected: vec!["{literal}"],
                found: "x".to_owned(),
            }
            .into())
        );
    }
}
