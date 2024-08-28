//!
//! The switch statement.
//!

pub mod case;

use std::collections::HashSet;

use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::keyword::Keyword;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::dialect::Dialect;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::expression::Expression;

use self::case::Case;

///
/// The Yul switch statement.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(bound = "P: serde::de::DeserializeOwned")]
pub struct Switch<P>
where
    P: Dialect,
{
    /// The location.
    pub location: Location,
    /// The expression being matched.
    pub expression: Expression,
    /// The non-default cases.
    pub cases: Vec<Case<P>>,
    /// The optional default case, if `cases` do not cover all possible values.
    pub default: Option<Block<P>>,
}

///
/// The parsing state.
///
pub enum State {
    /// After match expression.
    CaseOrDefaultKeyword,
    /// After `case`.
    CaseBlock,
    /// After `default`.
    DefaultBlock,
}

impl<P> Switch<P>
where
    P: Dialect,
{
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let mut token = crate::yul::parser::take_or_next(initial, lexer)?;
        let location = token.location;
        let mut state = State::CaseOrDefaultKeyword;

        let expression = Expression::parse(lexer, Some(token.clone()))?;
        let mut cases = Vec::new();
        let mut default = None;

        loop {
            match state {
                State::CaseOrDefaultKeyword => match lexer.peek()? {
                    _token @ Token {
                        lexeme: Lexeme::Keyword(Keyword::Case),
                        ..
                    } => {
                        token = _token;
                        state = State::CaseBlock;
                    }
                    _token @ Token {
                        lexeme: Lexeme::Keyword(Keyword::Default),
                        ..
                    } => {
                        token = _token;
                        state = State::DefaultBlock;
                    }
                    _token => {
                        token = _token;
                        break;
                    }
                },
                State::CaseBlock => {
                    lexer.next()?;
                    cases.push(Case::parse(lexer, None)?);
                    state = State::CaseOrDefaultKeyword;
                }
                State::DefaultBlock => {
                    lexer.next()?;
                    default = Some(Block::parse(lexer, None)?);
                    break;
                }
            }
        }

        if cases.is_empty() && default.is_none() {
            return Err(ParserError::InvalidToken {
                location: token.location,
                expected: vec!["case", "default"],
                found: token.lexeme.to_string(),
            }
            .into());
        }

        Ok(Self {
            location,
            expression,
            cases,
            default,
        })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        let mut libraries = HashSet::new();
        for case in self.cases.iter() {
            libraries.extend(case.get_missing_libraries());
        }
        if let Some(default) = &self.default {
            libraries.extend(default.get_missing_libraries());
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
    fn error_invalid_token_case() {
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
                    branch x {}
                    default {}
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
                location: Location::new(12, 21),
                expected: vec!["case", "default"],
                found: "branch".to_owned(),
            }
            .into())
        );
    }
}
