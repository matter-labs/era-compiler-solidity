//!
//! Kind of a [`DefinitionInfo`].
//!

use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::unary::UnaryOpType;
use crate::easycrypt::syntax::function::name::FunctionName;
use crate::easycrypt::syntax::proc::name::ProcName;

use super::attributes::Attributes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YulSpecial {
    Return,
    Revert,
    Stop,
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcKind {
    pub name: ProcName,
    pub attributes: Attributes,
}

/// Kind of a [`DefinitionInfo`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Function(FunctionName),
    Proc(ProcKind),
    BinOp(BinaryOpType),
    UnOp(UnaryOpType),
    Special(YulSpecial),
    Variable,
}
