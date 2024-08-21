//!
//! Transpilation of YUL literals.
//!
use anyhow::Error;

use crate::easycrypt::syntax::expression::call::FunctionCall;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::function::name::FunctionName;
use crate::easycrypt::syntax::literal::integer::IntegerLiteral;
use crate::easycrypt::syntax::literal::Literal;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::statement::expression::literal::Literal as YulLiteral;

impl Translator {
    ///
    /// Transpile an arbitrary YUL literal into an EasyCrypt literal.
    ///
    pub fn transpile_literal(lit: &YulLiteral) -> Result<Expression, Error> {
        let transpiled_integer = match &lit.inner {
            crate::yul::lexer::token::lexeme::literal::Literal::Boolean(b) => {
                if b == &crate::yul::lexer::token::lexeme::literal::boolean::Boolean::True {
                    Literal::Int(IntegerLiteral::Decimal {
                        inner: "1".to_string(),
                    })
                } else {
                    Literal::Int(IntegerLiteral::Decimal {
                        inner: "0".to_string(),
                    })
                }
            }
            crate::yul::lexer::token::lexeme::literal::Literal::Integer(i) => match i {
                crate::yul::lexer::token::lexeme::literal::integer::Integer::Decimal { inner } => {
                    Literal::Int(IntegerLiteral::Decimal {
                        inner: inner.to_string(),
                    })
                }
                crate::yul::lexer::token::lexeme::literal::integer::Integer::Hexadecimal {
                    inner,
                } => Literal::Int(IntegerLiteral::Decimal {
                    inner: crate::util::num::from_hex_literal(inner).to_string(),
                }),
            },

            crate::yul::lexer::token::lexeme::literal::Literal::String(s) => {
                Literal::String(s.to_string())
            }
        };

        let wrapper_call = FunctionCall {
            target: FunctionName {
                name: String::from("of_int"),
                module: Some(String::from("W256")),
                yul_name: None,
            },
            arguments: vec![Expression::Literal(transpiled_integer)],
        };
        Ok(Expression::ECall(wrapper_call))
    }
}
