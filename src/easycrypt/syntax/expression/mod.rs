use self::{binary::BinaryOpType, call::FunctionCall, unary::UnaryOpType};

use super::{literal::Literal, reference::Reference};

pub mod binary;
pub mod call;
pub mod unary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Unary(UnaryOpType, Box<Self>),
    Binary(BinaryOpType, Box<Self>, Box<Self>),
    ECall(FunctionCall),
    Literal(Literal),
    Reference(Reference),
    Tuple(Vec<Self>),
}

impl Expression {
    /// Pack two or more expressions in a tuple expression. A single expression
    /// is returned as-is, unpacked.
    pub fn pack_tuple(exprs: &Vec<Self>) -> Self {
        match exprs.len() {
            0 => panic!("Attempt to pack zero expressions in a tuple."),
            1 => exprs[0].clone(),
            _ => Self::Tuple(exprs.to_vec()),
        }
    }
}
