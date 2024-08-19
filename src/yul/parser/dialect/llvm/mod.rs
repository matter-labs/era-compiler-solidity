//!
//! LLVM-specific part of the parser.
//!

pub mod attributes;

use std::collections::BTreeSet;

use crate::yul::error::Error;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::Lexer;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::identifier::Identifier;

use self::attributes::get_llvm_attributes;

use super::Dialect;

/// LLVM-specific part of the parser.
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LLVMDialect {}

impl Dialect for LLVMDialect {
    type FunctionAttribute = era_compiler_llvm_context::Attribute;

    fn extract_attributes(
        identifier: &Identifier,
        _: &mut Lexer,
    ) -> Result<BTreeSet<Self::FunctionAttribute>, crate::yul::error::Error> {
        get_llvm_attributes(identifier)
    }

    fn sanitize_function(
        identifier: &Identifier,
        arguments: &mut Vec<Identifier>,
        location: Location,
        _lexer: &mut Lexer,
    ) -> Result<(), Error> {
        if identifier
            .inner
            .contains(era_compiler_llvm_context::EraVMFunction::ZKSYNC_NEAR_CALL_ABI_PREFIX)
        {
            if arguments.is_empty() {
                return Err(ParserError::InvalidNumberOfArguments {
                    location,
                    identifier: identifier.inner.clone(),
                    expected: 1,
                    found: arguments.len(),
                }
                .into());
            }

            arguments.remove(0);
        }
        if identifier.inner.contains(
            era_compiler_llvm_context::EraVMFunction::ZKSYNC_NEAR_CALL_ABI_EXCEPTION_HANDLER,
        ) && !arguments.is_empty()
        {
            return Err(ParserError::InvalidNumberOfArguments {
                location,
                identifier: identifier.inner.clone(),
                expected: 0,
                found: arguments.len(),
            }
            .into());
        }
        Ok(())
    }
}
