//!
//! Transpilation of YUL literals.
//!
use anyhow::Error;

use crate::easycrypt::syntax::literal::integer::IntegerLiteral;
use crate::easycrypt::syntax::literal::Literal;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::statement::expression::literal::Literal as YulLiteral;

impl Translator {
    /// Transpile an arbitrary YUL literal into an EasyCrypt literal.
    pub fn transpile_literal(lit: &YulLiteral) -> Result<Literal, Error> {
        match &lit.inner {
            crate::yul::lexer::token::lexeme::literal::Literal::Boolean(b) => {
                let is_true =
                    b == &crate::yul::lexer::token::lexeme::literal::boolean::Boolean::True;
                Ok(Literal::Bool(is_true))
            }
            crate::yul::lexer::token::lexeme::literal::Literal::Integer(i) => match i {
                crate::yul::lexer::token::lexeme::literal::integer::Integer::Decimal { inner } => {
                    Ok(Literal::Int(IntegerLiteral::Decimal {
                        inner: inner.to_string(),
                    }))
                }
                crate::yul::lexer::token::lexeme::literal::integer::Integer::Hexadecimal {
                    inner,
                } => Ok(Literal::Int(IntegerLiteral::Decimal {
                    inner: crate::util::num::from_hex_literal(inner).to_string(),
                })),
            },

            crate::yul::lexer::token::lexeme::literal::Literal::String(s) => {
                Ok(Literal::String(s.to_string()))
            }
        }
    }
}
