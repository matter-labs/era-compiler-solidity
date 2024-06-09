//!
//! Transpilation of Yul functions.
//!

use anyhow::Error;
use std::iter;

use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::r#type::context_kind::ContextKind;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::signature::Signature;
use crate::easycrypt::syntax::signature::SignatureKind;
use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::identifier::Identifier as YulIdentifier;
use crate::yul::parser::statement::function_definition::FunctionDefinition;
use crate::yul::path::tracker::PathTracker;

use super::block::Transformed as TransformedBlock;
use super::context::Context;
use super::definition_info::kind::Kind;

pub enum Translated {
    Function(Function),
    Proc(Proc),
}

impl Translator {
    /// Transpile a formal parameter of a YUL function, encountered in its signature.
    pub fn transpile_formal_parameter(&self, ident: &YulIdentifier) -> (Definition, Type) {
        let (name, typ) = Self::transpile_identifier(ident);
        (self.new_definition_here(&name, Some(typ.clone())), typ)
    }

    /// Transpile an arbitrary YUL function into an EasyCrypt function or procedure.
    pub fn transpile_function_definition(
        &mut self,
        fd: &FunctionDefinition,
        ctx: &Context,
    ) -> Result<(Context, Translated), Error> {
        let FunctionDefinition {
            identifier,
            arguments,
            result,
            body,
            ..
        } = fd;
        let kind = self
            .definitions
            .get_mut(&self.create_full_name(identifier))
            .unwrap()
            .kind
            .clone();
        self.tracker.enter_function(identifier);
        let formal_parameters = arguments
            .iter()
            .map(|ident| self.transpile_formal_parameter(ident))
            .collect();
        let (ctx, ec_block) = self.transpile_block(body, &ctx.clear_locals())?;
        let result_vars = self.bindings_to_definitions(result);
        let return_type: Type = Type::type_of_definitions(&result_vars);

        match kind {
            Kind::Function(_) => {
                match &ec_block.statements[0] {
                    Statement::EAssignment(_, expr) =>  {
            self.translate_to_function(formal_parameters, return_type, &ctx, identifier, expr)
                    },
                    _ => anyhow::bail!("Attempt to translate a YUL function into EasyCrypt function, but only translating to procedure is possible."),

                }
            },
            Kind::Proc(_) => {
            self.translate_to_procedure(
                formal_parameters,
                return_type,
                result_vars,
                ec_block,
                ctx,
                identifier,
            )
            }
            _ => anyhow::bail!("Malformed collection of definitions"),
        }
    }

    fn translate_to_procedure(
        &mut self,
        formal_parameters: Vec<(Definition, Type)>,
        return_type: Type,
        result_vars: Vec<Definition>,
        ec_block: TransformedBlock,
        ctx: Context,
        identifier: &str,
    ) -> Result<(Context, Translated), Error> {
        let signature = Signature {
            formal_parameters,
            return_type,
            kind: SignatureKind::Proc,
        };
        let statements = if signature.return_type != Type::Unit {
            let return_statement = Statement::Return(Expression::pack_tuple(
                &result_vars
                    .iter()
                    .map(|d| Expression::Reference(d.reference()))
                    .collect::<Vec<_>>(),
            ));
            ec_block
                .statements
                .iter()
                .chain(iter::once(&return_statement))
                .cloned()
                .collect()
        } else {
            ec_block.statements
        };
        let locals = result_vars
            .iter()
            .chain(ctx.locals.iter())
            .cloned()
            .collect();
        self.tracker.leave();
        Ok((
            ctx.clone(),
            Translated::Proc(Proc {
                name: identifier.to_string(),
                signature,
                locals,
                body: Block { statements },
                location: Some(self.here()),
            }),
        ))
    }

    fn translate_to_function(
        &mut self,
        formal_parameters: Vec<(Definition, Type)>,
        return_type: Type,
        ctx: &Context,
        identifier: &str,
        body_expr: &Expression,
    ) -> Result<(Context, Translated), Error> {
        let signature = Signature {
            formal_parameters,
            return_type,
            kind: SignatureKind::Function,
        };
        self.tracker.leave();
        Ok((
            ctx.clone(),
            Translated::Function(Function {
                name: identifier.to_string(),
                signature,
                body: body_expr.clone(),
                location: Some(self.here()),
            }),
        ))
    }
}
