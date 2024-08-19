//!
//! The block statement.
//!

use crate::create_wrapper;

use super::dialect::llvm::LLVMDialect;

pub mod assignment;
pub mod block;
pub mod code;
pub mod expression;
pub mod for_loop;
pub mod function_definition;
pub mod if_conditional;
pub mod object;
pub mod switch;
pub mod variable_declaration;

create_wrapper!(
    yul_syntax_tools::yul::parser::statement::Statement<LLVMDialect>,
    WrappedStatement
);
