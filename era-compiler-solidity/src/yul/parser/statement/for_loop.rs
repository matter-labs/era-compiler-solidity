//!
//! The for-loop statement.
//!

use std::collections::HashSet;

use era_compiler_llvm_context::IContext;

use crate::yul::error::Error;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::dialect::llvm::LLVMDialect;
use crate::yul::parser::dialect::Dialect;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::expression::Expression;

///
/// The Yul for-loop statement.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(bound = "P: serde::de::DeserializeOwned")]
pub struct ForLoop<P>
where
    P: Dialect,
{
    /// The location.
    pub location: Location,
    /// The index variables initialization block.
    pub initializer: Block<P>,
    /// The continue condition block.
    pub condition: Expression,
    /// The index variables mutating block.
    pub finalizer: Block<P>,
    /// The loop body.
    pub body: Block<P>,
}

impl<P> ForLoop<P>
where
    P: Dialect,
{
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;
        let location = token.location;

        let initializer = Block::parse(lexer, Some(token))?;

        let condition = Expression::parse(lexer, None)?;

        let finalizer = Block::parse(lexer, None)?;

        let body = Block::parse(lexer, None)?;

        Ok(Self {
            location,
            initializer,
            condition,
            finalizer,
            body,
        })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        let mut libraries = self.initializer.get_missing_libraries();
        libraries.extend(self.condition.get_missing_libraries());
        libraries.extend(self.finalizer.get_missing_libraries());
        libraries.extend(self.body.get_missing_libraries());
        libraries
    }
}

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for ForLoop<LLVMDialect>
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        self.initializer.into_llvm(context)?;

        let condition_block = context.append_basic_block("for_condition");
        let body_block = context.append_basic_block("for_body");
        let increment_block = context.append_basic_block("for_increment");
        let join_block = context.append_basic_block("for_join");

        context.build_unconditional_branch(condition_block)?;
        context.set_basic_block(condition_block);
        let condition = self
            .condition
            .into_llvm(context)?
            .expect("Always exists")
            .to_llvm()
            .into_int_value();
        let condition = context.builder().build_int_z_extend_or_bit_cast(
            condition,
            context.field_type(),
            "for_condition_extended",
        )?;
        let condition = context.builder().build_int_compare(
            inkwell::IntPredicate::NE,
            condition,
            context.field_const(0),
            "for_condition_compared",
        )?;
        context.build_conditional_branch(condition, body_block, join_block)?;

        context.push_loop(body_block, increment_block, join_block);

        context.set_basic_block(body_block);
        self.body.into_llvm(context)?;
        context.build_unconditional_branch(increment_block)?;

        context.set_basic_block(increment_block);
        self.finalizer.into_llvm(context)?;
        context.build_unconditional_branch(condition_block)?;

        context.pop_loop();
        context.set_basic_block(join_block);

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for ForLoop<LLVMDialect>
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        self.initializer.into_llvm(context)?;

        let condition_block = context.append_basic_block("for_condition");
        let body_block = context.append_basic_block("for_body");
        let increment_block = context.append_basic_block("for_increment");
        let join_block = context.append_basic_block("for_join");

        context.build_unconditional_branch(condition_block)?;
        context.set_basic_block(condition_block);
        let condition = self
            .condition
            .into_llvm_evm(context)?
            .expect("Always exists")
            .to_llvm()
            .into_int_value();
        let condition = context.builder().build_int_z_extend_or_bit_cast(
            condition,
            context.field_type(),
            "for_condition_extended",
        )?;
        let condition = context.builder().build_int_compare(
            inkwell::IntPredicate::NE,
            condition,
            context.field_const(0),
            "for_condition_compared",
        )?;
        context.build_conditional_branch(condition, body_block, join_block)?;

        context.push_loop(body_block, increment_block, join_block);

        context.set_basic_block(body_block);
        self.body.into_llvm(context)?;
        context.build_unconditional_branch(increment_block)?;

        context.set_basic_block(increment_block);
        self.finalizer.into_llvm(context)?;
        context.build_unconditional_branch(condition_block)?;

        context.pop_loop();
        context.set_basic_block(join_block);

        Ok(())
    }
}
