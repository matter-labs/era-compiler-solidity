//!
//! Transpilation of arbitrary YUL statements.
//!

pub mod assignment;
pub mod for_loop;
pub mod if_conditional;
pub mod switch;
pub mod variable_declaration;

use std::iter;

use anyhow::Error;

use super::context::Context;
use super::definition_info::kind::proc_kind::ProcKind;
use super::function;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::block;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::statement::Statement as YulStatement;

pub enum Transformed {
    Statements(Vec<Statement>),
    Function(Function),
    Proc(Proc),
}

impl Transformed {
    pub fn as_statements(&self) -> Option<&Vec<Statement>> {
        if let Self::Statements(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Translator {
    /// Transpile an arbitrary YUL statement.
    pub fn transpile_statement(
        &mut self,
        stmt: &YulStatement,
        ctx: &Context,
    ) -> Result<(Context, Transformed), Error> {
        match stmt {
            YulStatement::Object(_) => todo!(),
            YulStatement::Code(_code) => todo!(),
            YulStatement::Block(block) => {
                let (new_ctx, block::Transformed { statements }) =
                    self.transpile_block(block, ctx)?;
                Ok((new_ctx, Transformed::Statements(statements)))
            }
            YulStatement::Expression(expr) => match self.transpile_expression_root(expr, ctx)? {
                super::expression::Transformed::Expression(Expression::Reference(_), ectx) => Ok((
                    ctx.add_locals(&ectx.locals),
                    Transformed::Statements(ectx.assignments.to_vec()),
                )),
                super::expression::Transformed::Expression(result, ectx) => Ok((
                    ctx.add_locals(&ectx.locals),
                    Transformed::Statements(
                        ectx.assignments
                            .iter()
                            .chain(iter::once(&Statement::Expression(result)))
                            .cloned()
                            .collect(),
                    ),
                )),
                super::expression::Transformed::Statements(statements, ectx, ctx) => {
                    let result = ectx
                        .assignments
                        .iter()
                        .chain(statements.iter())
                        .cloned()
                        .collect();
                    Ok((
                        ctx.add_locals(&ectx.locals),
                        Transformed::Statements(result),
                    ))
                }
            },
            YulStatement::FunctionDefinition(fd) => {
                let (ctx, translation_result) = self.transpile_function_definition(fd, ctx)?;
                match translation_result {
                    function::Translated::Function(ec_function) => {
                        self.tracker.add(
                            &ec_function.name.to_string(),
                            &DefinitionInfo {
                                kind: Kind::Function(ec_function.name.clone()),
                                full_name: self.create_full_name(&ec_function.name.to_string()),
                                r#type: ec_function.signature.get_type(),
                            },
                        );
                        let mut new_ctx = ctx.clone();
                        new_ctx
                            .module
                            .add_def(TopDefinition::Function(ec_function.clone()));
                        Ok((new_ctx, Transformed::Function(ec_function)))
                    }
                    function::Translated::Proc(ec_procedure) => {
                        self.tracker.add(
                            &ec_procedure.name.to_string(),
                            &DefinitionInfo {
                                kind: Kind::Proc(ProcKind {
                                    name: ec_procedure.name.clone(),
                                    attributes: Default::default(),
                                }),
                                full_name: self.create_full_name(&ec_procedure.name.to_string()),
                                r#type: ec_procedure.signature.get_type(),
                            },
                        );
                        let mut new_ctx = ctx.clone();
                        new_ctx
                            .module
                            .add_def(TopDefinition::Proc(ec_procedure.clone()));
                        Ok((new_ctx, Transformed::Proc(ec_procedure)))
                    }
                }
            }
            YulStatement::VariableDeclaration(vd) => self.transpile_variable_declaration(vd, ctx),
            YulStatement::Assignment(assignment) => self.transpile_assignment(assignment, ctx),
            YulStatement::IfConditional(conditional) => self.transpile_if(conditional, ctx),
            YulStatement::Switch(switch) => self.transpile_switch(switch, ctx),
            YulStatement::ForLoop(for_loop) => self.transpile_for_loop(for_loop, ctx),
            YulStatement::Continue(_) => {
                anyhow::bail!("The `continue` statement is not supported.")
            }
            YulStatement::Break(_) => anyhow::bail!("The `break` statement is not supported."),
            YulStatement::Leave(_) => anyhow::bail!("The `leave` statement is not supported."),
        }
    }
}
