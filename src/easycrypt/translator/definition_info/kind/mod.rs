//!
//! Kind of a [`DefinitionInfo`].
//!

pub mod proc_kind;

use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::unary::UnaryOpType;
use crate::easycrypt::syntax::function::name::FunctionName;

use self::proc_kind::ProcKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YulSpecial {
    Return,
    Revert,
    Stop,
    Invalid,
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
