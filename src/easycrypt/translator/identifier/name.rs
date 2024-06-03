//!
//! Transpilation of the names of YUL identifiers.
//!

use anyhow::Error;

use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::definition_info::get_definition;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::kind::ProcKind;
use crate::easycrypt::translator::identifier::Translated;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;

impl Translator {
    /// Transpile an arbitrary YUL identifier's name, which can be a
    /// user-defined custom name or a predefined name like `lt` of `call`.
    pub fn transpile_name(&mut self, _ctx: &Context, name: &YulName) -> Result<Translated, Error> {
        match &get_definition(&self.definitions, name, &self.here())?.kind {
            Kind::Function(fun) => Ok(Translated::Function(fun.clone())),
            Kind::Proc(ProcKind { name, .. }) => Ok(Translated::Proc(name.clone())),
            Kind::BinOp(op_type) => Ok(Translated::BinOp(op_type.clone())),
            Kind::UnOp(op_type) => Ok(Translated::UnOp(op_type.clone())),
            Kind::Special(special) => Ok(Translated::Special(special.clone())),
            Kind::Variable => unreachable!("Internal error."),
        }
    }
}
