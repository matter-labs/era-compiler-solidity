//!
//! EasyCrypt AST nodes containing different kinds of expressions.
//!

pub mod binary;
pub mod call;
pub mod unary;

use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::call::FunctionCall;
use crate::easycrypt::syntax::expression::unary::UnaryOpType;
use crate::easycrypt::syntax::literal::Literal;
use crate::easycrypt::syntax::reference::Reference;

///
/// EasyCrypt AST nodes containing different kinds of expressions. Expressions
/// are a syntactic category whose terms are computed to a value in a pure way,
/// without side effects.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    /// Unary expressions, like `-x`.
    Unary(UnaryOpType, Box<Self>),
    /// Binary expressions, like `a-x`.
    Binary(BinaryOpType, Box<Self>, Box<Self>),
    /// Function calls, like `f(x)`.
    ECall(FunctionCall),
    /// Literals, like `42` or `"hello"`.
    Literal(Literal),
    /// References to previously defined variables, like `x`
    Reference(Reference),
    /// Tuples, like `(42, x)`.
    Tuple(Vec<Self>),
}

impl Expression {
    ///
    /// Pack two or more expressions in a tuple expression. A single expression
    /// is returned as-is, unpacked.
    ///
    pub fn pack_tuple(exprs: &[Self]) -> Self {
        match exprs.len() {
            0 => panic!("Attempt to pack zero expressions in a tuple."),
            1 => exprs[0].clone(),
            _ => Self::Tuple(exprs.to_vec()),
        }
    }
}
