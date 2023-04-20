//!
//! The YUL source code literal.
//!

use inkwell::values::BasicValue;
use num::Num;
use num::One;
use num::Zero;

use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::literal::boolean::Boolean as BooleanLiteral;
use crate::yul::lexer::token::lexeme::literal::integer::Integer as IntegerLiteral;
use crate::yul::lexer::token::lexeme::literal::Literal as LexicalLiteral;
use crate::yul::lexer::token::lexeme::symbol::Symbol;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::r#type::Type;

///
/// Represents a literal in YUL without differentiating its type.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    /// The location.
    pub location: Location,
    /// The lexical literal.
    pub inner: LexicalLiteral,
    /// The type, if it has been explicitly specified.
    pub yul_type: Option<Type>,
}

impl Literal {
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;

        let (location, literal) = match token {
            Token {
                lexeme: Lexeme::Literal(literal),
                location,
                ..
            } => (location, literal),
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["{literal}"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };

        let yul_type = match lexer.peek()? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Colon),
                ..
            } => {
                lexer.next()?;
                Some(Type::parse(lexer, None)?)
            }
            _ => None,
        };

        Ok(Self {
            location,
            inner: literal,
            yul_type,
        })
    }

    ///
    /// Converts the literal into its LLVM.
    ///
    pub fn into_llvm<'ctx, D>(
        self,
        context: &compiler_llvm_context::Context<'ctx, D>,
    ) -> compiler_llvm_context::Argument<'ctx>
    where
        D: compiler_llvm_context::Dependency,
    {
        match self.inner {
            LexicalLiteral::Boolean(inner) => {
                let value = self
                    .yul_type
                    .unwrap_or_default()
                    .into_llvm(context)
                    .const_int(
                        match inner {
                            BooleanLiteral::False => 0,
                            BooleanLiteral::True => 1,
                        },
                        false,
                    )
                    .as_basic_value_enum();

                let constant = match inner {
                    BooleanLiteral::False => num::BigUint::zero(),
                    BooleanLiteral::True => num::BigUint::one(),
                };

                compiler_llvm_context::Argument::new_with_constant(value, constant)
            }
            LexicalLiteral::Integer(inner) => {
                let r#type = self.yul_type.unwrap_or_default().into_llvm(context);
                let value = match inner {
                    IntegerLiteral::Decimal { ref inner } => r#type.const_int_from_string(
                        inner.as_str(),
                        inkwell::types::StringRadix::Decimal,
                    ),
                    IntegerLiteral::Hexadecimal { ref inner } => r#type.const_int_from_string(
                        &inner["0x".len()..],
                        inkwell::types::StringRadix::Hexadecimal,
                    ),
                }
                .expect("The value is valid")
                .as_basic_value_enum();

                let constant = match inner {
                    IntegerLiteral::Decimal { ref inner } => {
                        num::BigUint::from_str_radix(inner.as_str(), compiler_common::BASE_DECIMAL)
                    }
                    IntegerLiteral::Hexadecimal { ref inner } => num::BigUint::from_str_radix(
                        &inner["0x".len()..],
                        compiler_common::BASE_HEXADECIMAL,
                    ),
                }
                .expect("Always valid");

                compiler_llvm_context::Argument::new_with_constant(value, constant)
            }
            LexicalLiteral::String(inner) => {
                let string = inner.to_string();
                let r#type = self.yul_type.unwrap_or_default().into_llvm(context);

                let mut hex_string = String::with_capacity(compiler_common::BYTE_LENGTH_FIELD * 2);
                let mut index = 0;
                loop {
                    if index >= string.len() {
                        break;
                    }

                    if string[index..].starts_with("\\x") {
                        hex_string.push_str(&string[index + 2..index + 4]);
                        index += 4;
                    } else if string[index..].starts_with("\\t") {
                        hex_string.push_str("09");
                        index += 2;
                    } else if string[index..].starts_with("\\n") {
                        hex_string.push_str("0a");
                        index += 2;
                    } else if string[index..].starts_with("\\r") {
                        hex_string.push_str("0d");
                        index += 2;
                    } else {
                        hex_string.push_str(format!("{:02x}", string.as_bytes()[index]).as_str());
                        index += 1;
                    }
                }

                if hex_string.len() > compiler_common::BYTE_LENGTH_FIELD * 2 {
                    return compiler_llvm_context::Argument::new_with_original(
                        r#type.const_zero().as_basic_value_enum(),
                        string,
                    );
                }
                if string.len() < compiler_common::BYTE_LENGTH_FIELD {
                    hex_string.push_str(
                        "00".repeat(compiler_common::BYTE_LENGTH_FIELD - string.len())
                            .as_str(),
                    );
                }

                let value = r#type
                    .const_int_from_string(
                        hex_string.as_str(),
                        inkwell::types::StringRadix::Hexadecimal,
                    )
                    .expect("The value is valid")
                    .as_basic_value_enum();
                compiler_llvm_context::Argument::new_with_original(value, string)
            }
        }
    }
}
