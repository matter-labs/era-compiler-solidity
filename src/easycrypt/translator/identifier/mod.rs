//!
//! Transpilation of YUL identifiers.
//!

pub mod name;
pub mod special;

use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::unary::UnaryOpType;
use crate::easycrypt::syntax::function::name::FunctionName;

use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::Name;

use crate::easycrypt::translator::Translator;
use crate::yul::parser::identifier::Identifier as YulIdentifier;

use self::special::YulSpecial;

pub enum Translated {
    Function(FunctionName),
    Proc(ProcName),
    ProcOrFunction(Name),
    BinOp(BinaryOpType),
    UnOp(UnaryOpType),
    Special(YulSpecial),
}

impl Translator {
    /// Transpile an arbitrary YUL identifier.
    pub fn transpile_identifier(&mut self, ident: &YulIdentifier) -> (Name, Type) {
        let new_type = match &ident.r#type {
            Some(typ) => Self::transpile_type(typ).unwrap(),
            None => Self::DEFAULT_TYPE,
        };

        (ident.inner.to_string(), new_type)
    }
}
