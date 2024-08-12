//!
//! The YUL code.
//!

use std::collections::HashSet;

use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::keyword::Keyword;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::dialect::llvm::LLVMDialect;
use crate::yul::parser::dialect::Dialect;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::statement::block::Block;

///
/// The YUL code entity, which is the first block of the object.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(bound = "P: serde::de::DeserializeOwned")]
pub struct Code<P>
where
    P: Dialect,
{
    /// The location.
    pub location: Location,
    /// The main block.
    pub block: Block<P>,
}

impl<P> Code<P>
where
    P: Dialect,
{
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;

        let location = match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Code),
                location,
                ..
            } => location,
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["code"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };

        let block = Block::parse(lexer, None)?;

        Ok(Self { location, block })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        self.block.get_missing_libraries()
    }
}

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for Code<LLVMDialect>
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        self.block.into_llvm(context)?;

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for Code<LLVMDialect>
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        self.block.into_llvm(context)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::yul::lexer::token::location::Location;
    use crate::yul::lexer::Lexer;
    use crate::yul::parser::dialect::llvm::LLVMDialect;
    use crate::yul::parser::error::Error;
    use crate::yul::parser::statement::object::Object;

    #[test]
    fn error_invalid_token_code() {
        let input = r#"
object "Test" {
    data {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                return(0, 0)
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<LLVMDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidToken {
                location: Location::new(3, 5),
                expected: vec!["code"],
                found: "data".to_owned(),
            }
            .into())
        );
    }
}
