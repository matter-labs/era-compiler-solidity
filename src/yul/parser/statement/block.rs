//!
//! The source code block.
//!

use era_compiler_llvm_context::IContext;
use yul_syntax_tools::yul::parser::statement::Statement;

use crate::create_wrapper;
use crate::yul::parser::dialect::llvm::LLVMDialect;
use crate::yul::parser::wrapper::Wrap as _;

create_wrapper!(
    yul_syntax_tools::yul::parser::statement::block::Block<LLVMDialect>,
    WrappedBlock
);

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for WrappedBlock
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        let term = self.0;
        let current_function = context.current_function().borrow().name().to_owned();
        let current_block = context.basic_block();

        let mut functions = Vec::with_capacity(term.statements.len());
        let mut local_statements = Vec::with_capacity(term.statements.len());

        for statement in term.statements.into_iter() {
            match statement {
                Statement::FunctionDefinition(statement) => {
                    statement.clone().wrap().declare(context)?;
                    functions.push(statement);
                }
                statement => local_statements.push(statement),
            }
        }

        for function in functions.into_iter() {
            function.wrap().into_llvm(context)?;
        }

        context.set_current_function(current_function.as_str())?;
        context.set_basic_block(current_block);
        for statement in local_statements.into_iter() {
            if context.basic_block().get_terminator().is_some() {
                break;
            }

            match statement {
                Statement::Block(block) => {
                    block.wrap().into_llvm(context)?;
                }
                Statement::Expression(expression) => {
                    expression.wrap().into_llvm(context)?;
                }
                Statement::VariableDeclaration(statement) => statement.wrap().into_llvm(context)?,
                Statement::Assignment(statement) => statement.wrap().into_llvm(context)?,
                Statement::IfConditional(statement) => statement.wrap().into_llvm(context)?,
                Statement::Switch(statement) => statement.wrap().into_llvm(context)?,
                Statement::ForLoop(statement) => statement.wrap().into_llvm(context)?,
                Statement::Continue(_location) => {
                    context.build_unconditional_branch(context.r#loop().continue_block)?;
                    break;
                }
                Statement::Break(_location) => {
                    context.build_unconditional_branch(context.r#loop().join_block)?;
                    break;
                }
                Statement::Leave(_location) => {
                    context.build_unconditional_branch(
                        context.current_function().borrow().return_block(),
                    )?;
                    break;
                }
                statement => anyhow::bail!(
                    "{} Unexpected local statement: {statement:?}",
                    statement.location(),
                ),
            }
        }

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for WrappedBlock
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        let current_function = context.current_function().borrow().name().to_owned();
        let current_block = context.basic_block();

        let mut functions = Vec::with_capacity(self.0.statements.len());
        let mut local_statements = Vec::with_capacity(self.0.statements.len());

        for statement in self.0.statements.into_iter() {
            match statement {
                Statement::FunctionDefinition(statement) => {
                    statement.clone().wrap().declare(context)?;
                    functions.push(statement);
                }
                statement => local_statements.push(statement),
            }
        }

        for function in functions.into_iter() {
            function.wrap().into_llvm(context)?;
        }

        context.set_current_function(current_function.as_str())?;
        context.set_basic_block(current_block);
        for statement in local_statements.into_iter() {
            if context.basic_block().get_terminator().is_some() {
                break;
            }

            match statement {
                Statement::Block(block) => {
                    block.wrap().into_llvm(context)?;
                }
                Statement::Expression(expression) => {
                    expression.wrap().into_llvm_evm(context)?;
                }
                Statement::VariableDeclaration(statement) => statement.wrap().into_llvm(context)?,
                Statement::Assignment(statement) => statement.wrap().into_llvm(context)?,
                Statement::IfConditional(statement) => statement.wrap().into_llvm(context)?,
                Statement::Switch(statement) => statement.wrap().into_llvm(context)?,
                Statement::ForLoop(statement) => statement.wrap().into_llvm(context)?,
                Statement::Continue(_location) => {
                    context.build_unconditional_branch(context.r#loop().continue_block)?;
                    break;
                }
                Statement::Break(_location) => {
                    context.build_unconditional_branch(context.r#loop().join_block)?;
                    break;
                }
                Statement::Leave(_location) => {
                    context.build_unconditional_branch(
                        context.current_function().borrow().return_block(),
                    )?;
                    break;
                }
                statement => anyhow::bail!(
                    "{} Unexpected local statement: {statement:?}",
                    statement.location(),
                ),
            }
        }

        Ok(())
    }
}
