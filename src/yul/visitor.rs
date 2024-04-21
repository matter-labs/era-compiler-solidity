use crate::yul::parser::statement::{
    assignment::Assignment,
    block::Block,
    code::Code,
    expression::{
        function_call::{name::Name, FunctionCall},
        literal::Literal,
        Expression,
    },
    for_loop::ForLoop,
    function_definition::FunctionDefinition,
    if_conditional::IfConditional,
    object::Object,
    switch::Switch,
    variable_declaration::VariableDeclaration,
    Statement,
};

pub trait YulVisitor {
    fn visit_switch(&mut self, s: &Switch);
    fn visit_object(&mut self, obj: &Object);
    fn visit_for_loop(&mut self, for_loop: &ForLoop);
    fn visit_variable_declaration(&mut self, vd: &VariableDeclaration);
    fn visit_function_definition(&mut self, fd: &FunctionDefinition);
    fn visit_name(&mut self, name: &Name);
    fn visit_functioncall(&mut self, call: &FunctionCall);
    fn visit_if_conditional(&mut self, if_conditional: &IfConditional);
    fn visit_literal(&mut self, lit: &Literal);
    fn visit_expression(&mut self, expr: &Expression);
    fn visit_assignment(&mut self, assignment: &Assignment);
    fn visit_statement(&mut self, stmt: &Statement);
    fn visit_block(&mut self, block: &Block);
    fn visit_code(&mut self, code: &Code);
}
