//!
//! Visitor for EasyCrypt AST
//!

use super::syntax::{
    BinaryOpType, Block, Definition, Expression, Function, FunctionCall, FunctionName,
    IntegerLiteral, Literal, Module, ModuleDefinition, Proc, ProcCall, ProcName, Reference,
    Signature, Statement, Type, UnaryOpType,
};

pub trait Visitor {
    fn visit_binary_op_type(&mut self, op: &BinaryOpType);
    fn visit_block(&mut self, block: &Block);
    fn visit_definition(&mut self, definition: &Definition);
    fn visit_expression(&mut self, expression: &Expression);
    fn visit_function(&mut self, function: &Function);
    fn visit_function_call(&mut self, call: &FunctionCall);
    fn visit_function_name(&mut self, proc_name: &FunctionName);
    fn visit_integer_literal(&mut self, int_literal: &IntegerLiteral);
    fn visit_literal(&mut self, literal: &Literal);
    fn visit_module(&mut self, module: &Module);
    fn visit_module_definition(&mut self, module: &ModuleDefinition);
    fn visit_proc_call(&mut self, pcall: &ProcCall);
    fn visit_proc(&mut self, proc: &Proc);
    fn visit_proc_name(&mut self, proc_name: &ProcName);
    fn visit_reference(&mut self, reference: &Reference);
    fn visit_signature(&mut self, signature: &Signature);
    fn visit_statement(&mut self, statement: &Statement);
    fn visit_type(&mut self, r#type: &Type);
    fn visit_unary_op_type(&mut self, op: &UnaryOpType);
}
