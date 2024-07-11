//!
//! Transpilation of Yul functions.
//!

use anyhow::Error;
use std::iter;

use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::function::name::FunctionName;
use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::signature::Signature;
use crate::easycrypt::syntax::signature::SignatureKind;
use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::identifier::Identifier as YulIdentifier;
use crate::yul::parser::statement::function_definition::FunctionDefinition;
use crate::yul::path::full_name::FullName;
use crate::yul::path::tracker::PathTracker;

use super::block::Transformed as TransformedBlock;
use super::context::Context;
use super::definition_info::kind::proc_kind::state_formal_parameters;
use super::definition_info::kind::proc_kind::state_return_vars;
use super::definition_info::kind::Kind;
use super::definition_info::DefinitionInfo;

pub enum Translated {
    Function(Function),
    Proc(Proc),
}

impl Translator {
    fn all_formal_parameters(
        &self,
        yul_parameters: &[YulIdentifier],
        proc_definition: &DefinitionInfo,
    ) -> Vec<(Definition, Type)> {
        let extra_parameters_of_state = state_formal_parameters(proc_definition);

        yul_parameters
            .iter()
            .map(|ident| self.transpile_formal_parameter(ident))
            .chain(extra_parameters_of_state.iter().cloned())
            .collect::<Vec<_>>()
    }

    fn all_result_variables(
        &self,
        yul_results: &[YulIdentifier],
        proc_definition: &DefinitionInfo,
    ) -> Vec<Definition> {
        let binding = state_return_vars(proc_definition);
        let extra_results_of_state = binding.iter().map(|(def, _)| def.clone());

        self.bindings_to_definitions(yul_results)
            .iter()
            .cloned()
            .chain(extra_results_of_state)
            .collect::<Vec<_>>()
    }

    fn transpile_formal_parameter(&self, ident: &YulIdentifier) -> (Definition, Type) {
        let typ = ident
            .r#type
            .clone()
            .map(|t| Self::transpile_type(&t).unwrap());
        (
            self.new_definition_here(&ident.inner, typ.clone()),
            typ.unwrap_or(Type::DEFAULT.clone()),
        )
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
        let full_name = self.create_full_name(identifier);
        self.functions_stack.push(full_name.clone());
        let definition = self.definitions.get(&full_name).unwrap();

        let kind = definition.kind.clone();
        self.tracker.enter_function(identifier);

        let formal_parameters = self.all_formal_parameters(arguments, definition);
        let result_vars = self.all_result_variables(result, definition);
        let return_type: Type = Type::type_of_definitions(result_vars.as_slice());

        let (ctx, ec_block) = self.transpile_block(body, &ctx.clear_locals())?;
        match kind {
            // FIXME ugly
            Kind::Function(_) => {
                match &ec_block.statements[0] {
                    Statement::EAssignment(_, expr) =>  {
                        self.translate_to_function(formal_parameters, return_type, &ctx, identifier, &full_name, expr)
                    },
                    _ => anyhow::bail!("Attempt to translate a YUL function into EasyCrypt function, but only translating to procedure is possible."),

                }
            },
            Kind::Proc(_) => {
                self.translate_to_procedure(
                    &formal_parameters,
                    return_type,
                    result_vars,
                    ec_block,
                    ctx,
                    &full_name,
                    identifier,
                )
            }
            _ => anyhow::bail!("Malformed collection of definitions"),
        }
    }

    fn translate_to_procedure(
        &mut self,
        formal_parameters: &[(Definition, Type)],
        return_type: Type,
        result_vars: Vec<Definition>,
        ec_block: TransformedBlock,
        ctx: Context,
        yul_name: &FullName,
        identifier: &str,
    ) -> Result<(Context, Translated), Error> {
        let signature = Signature {
            formal_parameters: formal_parameters.to_vec(),
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
            .filter(|def| !formal_parameters.iter().any(|param| param.0 == **def))
            .chain(ctx.locals.iter())
            .cloned()
            .collect();
        self.tracker.leave();
        self.functions_stack.pop();
        Ok((
            ctx.clone(),
            Translated::Proc(Proc {
                name: ProcName {
                    name: identifier.to_string(),
                    module: None,
                    yul_name: Some(yul_name.clone()),
                },
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
        yul_name: &FullName,
        body_expr: &Expression,
    ) -> Result<(Context, Translated), Error> {
        let signature = Signature {
            formal_parameters,
            return_type,
            kind: SignatureKind::Function,
        };
        self.tracker.leave();
        self.functions_stack.pop();
        Ok((
            ctx.clone(),
            Translated::Function(Function {
                name: FunctionName {
                    name: identifier.to_string(),
                    module: None,
                    yul_name: Some(yul_name.clone()),
                },
                signature,
                body: body_expr.clone(),
                location: Some(self.here()),
            }),
        ))
    }
}
