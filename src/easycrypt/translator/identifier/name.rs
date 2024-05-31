//!
//! Transpilation of the names of YUL identifiers.
//!

use anyhow::Error;

use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::identifier::Translated;
use crate::easycrypt::translator::yul_analyzers::functions::kind::derive_kind;
use crate::easycrypt::translator::yul_analyzers::functions::kind::FunctionKind;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;

impl Translator {
    /// Transpile an arbitrary YUL identifier's name, which can be a
    /// user-defined custom name or a predefined name like `lt` of `call`.
    pub fn transpile_name(&mut self, _ctx: &Context, name: &YulName) -> Result<Translated, Error> {
        match derive_kind(&self.definitions, name, &self.here())? {
            FunctionKind::Function(fun) => Ok(Translated::Function(fun)),
            FunctionKind::Proc(proc) => Ok(Translated::Proc(proc)),
            FunctionKind::BinOp(op_type) => Ok(Translated::BinOp(op_type)),
            FunctionKind::UnOp(op_type) => Ok(Translated::UnOp(op_type)),
            FunctionKind::Special(special) => Ok(Translated::Special(special)),
        }
    }
}
