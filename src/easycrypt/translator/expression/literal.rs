//!
//! Transpilation of YUL literals.
//!

use crate::easycrypt::syntax::literal::integer::IntegerLiteral;
use crate::easycrypt::syntax::literal::Literal;
use crate::yul::parser::statement::expression::literal::Literal as YulLiteral;
use crate::Translator;

impl Translator {
    /// Transpile an arbitrary YUL literal into an EasyCrypt literal.
    pub fn transpile_literal(lit: &YulLiteral) -> Literal {
        match &lit.inner {
            crate::yul::lexer::token::lexeme::literal::Literal::Boolean(b) => {
                let is_true =
                    b == &crate::yul::lexer::token::lexeme::literal::boolean::Boolean::True;
                Literal::Bool(is_true)
            }
            crate::yul::lexer::token::lexeme::literal::Literal::Integer(i) => match i {
                crate::yul::lexer::token::lexeme::literal::integer::Integer::Decimal { inner } => {
                    Literal::Int(IntegerLiteral::Decimal {
                        inner: inner.to_string(),
                    })
                }
                crate::yul::lexer::token::lexeme::literal::integer::Integer::Hexadecimal {
                    inner,
                } => Literal::Int(IntegerLiteral::Hexadecimal {
                    inner: inner.to_string(),
                }),
            },

            crate::yul::lexer::token::lexeme::literal::Literal::String(s) => {
                Literal::String(s.to_string())
            }
        }
    }
}
