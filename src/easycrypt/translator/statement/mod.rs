pub mod assignment;
pub mod variable_declaration;

use std::iter;

use anyhow::Error;

use super::context::Context;
use super::function;
use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::block;
use crate::yul::parser::statement::Statement as YulStatement;
use crate::Translator;

pub enum Transformed {
    Statements(Vec<Statement>),
    Function(Function),
    Proc(Proc),
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
            YulStatement::Expression(expr) => {
                let (result, ectx) = self.transpile_expression_root(expr, ctx)?;
                Ok((
                    ctx.add_locals(&ectx.locals),
                    Transformed::Statements(
                        ectx.assignments
                            .iter()
                            .chain(iter::once(&Statement::Expression(result)))
                            .cloned()
                            .collect(),
                    ),
                ))
            }
            YulStatement::FunctionDefinition(fd) => {
                let (ctx, translation_result) = self.transpile_function_definition(fd, ctx)?;
                match translation_result {
                    function::Translated::Function(fd) => {
                        let mut new_ctx = ctx.clone();
                        new_ctx.module.add_def(TopDefinition::Function(fd.clone()));
                        Ok((new_ctx, Transformed::Function(fd)))
                    }
                    function::Translated::Proc(pd) => {
                        let mut new_ctx = ctx.clone();
                        new_ctx.module.add_def(TopDefinition::Proc(pd.clone()));
                        Ok((new_ctx, Transformed::Proc(pd)))
                    }
                }
            }
            YulStatement::VariableDeclaration(vd) => self.transpile_variable_declaration(vd, ctx),
            YulStatement::Assignment(assignment) => self.transpile_assignment(assignment, ctx),
            YulStatement::IfConditional(_) => todo!(),
            YulStatement::Switch(_) => todo!(),
            YulStatement::ForLoop(_) => todo!(),
            YulStatement::Continue(_) => todo!(),
            YulStatement::Break(_) => todo!(),
            YulStatement::Leave(_) => todo!(),
        }
    }
}
