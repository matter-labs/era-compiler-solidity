//!
//! Implementation of a visitor pattern for YUL syntax tree.
//!

use crate::yul::parser::statement::assignment::Assignment;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::code::Code;
use crate::yul::parser::statement::expression::function_call::name::Name;
use crate::yul::parser::statement::expression::function_call::FunctionCall;
use crate::yul::parser::statement::expression::literal::Literal;
use crate::yul::parser::statement::expression::Expression;
use crate::yul::parser::statement::for_loop::ForLoop;
use crate::yul::parser::statement::function_definition::FunctionDefinition;
use crate::yul::parser::statement::if_conditional::IfConditional;
use crate::yul::parser::statement::object::Object;
use crate::yul::parser::statement::switch::Switch;
use crate::yul::parser::statement::variable_declaration::VariableDeclaration;
use crate::yul::parser::statement::Statement;

/// Visitor for YUL syntax tree.
pub trait Visitor {
    /// Visit `switch` statement in YUL syntax tree.
    fn visit_switch(&mut self, switch: &Switch);
    /// Visit YUL object in YUL syntax tree.
    fn visit_object(&mut self, object: &Object);
    /// Visit `for` statement in YUL syntax tree.
    fn visit_for_loop(&mut self, for_loop: &ForLoop);
    /// Visit a variable declaration in YUL syntax tree: `var x` or `var x = <initializer>`.
    fn visit_variable_declaration(&mut self, variable_definition: &VariableDeclaration);
    /// Visit a function definition in YUL syntax tree.
    fn visit_function_definition(&mut self, function_definition: &FunctionDefinition);
    /// Visit an identifier in YUL syntax tree: a user defined one, or one of the predefined set like `lt`.
    fn visit_name(&mut self, name: &Name);
    /// Visit a function call in YUL syntax tree.
    fn visit_function_call(&mut self, call: &FunctionCall);
    /// Visit an `if` statement in YUL syntax tree.
    fn visit_if_conditional(&mut self, if_conditional: &IfConditional);
    /// Visit a literal (e.g. integer) in YUL syntax tree.
    fn visit_literal(&mut self, lit: &Literal);
    /// Visit an arbitrary YUL expression in YUL syntax tree.
    fn visit_expression(&mut self, expr: &Expression);
    /// Visit an assignment in YUL syntax tree.
    fn visit_assignment(&mut self, assignment: &Assignment);
    /// Visit an arbitrary statement in YUL syntax tree.
    fn visit_statement(&mut self, stmt: &Statement);
    /// Visit a block of statements in YUL syntax tree.
    fn visit_block(&mut self, block: &Block);
    /// Visit a `code` block of an object in YUL syntax tree.
    fn visit_code(&mut self, code: &Code);
}
