// //!
// //! Default implementation for visitor pattern for YUL syntax tree.
// //!

// use super::Visitor;
// use crate::yul::parser::statement::assignment::Assignment;
// use crate::yul::parser::statement::block::Block;
// use crate::yul::parser::statement::code::Code;
// use crate::yul::parser::statement::expression::function_call::name::Name;
// use crate::yul::parser::statement::expression::function_call::FunctionCall;
// use crate::yul::parser::statement::expression::literal::Literal;
// use crate::yul::parser::statement::expression::Expression;
// use crate::yul::parser::statement::for_loop::ForLoop;
// use crate::yul::parser::statement::function_definition::FunctionDefinition;
// use crate::yul::parser::statement::if_conditional::IfConditional;
// use crate::yul::parser::statement::object::Object;
// use crate::yul::parser::statement::switch::Switch;
// use crate::yul::parser::statement::variable_declaration::VariableDeclaration;
// use crate::yul::parser::statement::Statement;

// pub struct VisitorDefaultImpl<'a, T>(pub &'a mut T)
// where
//     T: Visitor;

// impl<'a, T> Visitor for VisitorDefaultImpl<'a, T>
// where
//     T: Visitor,
// {
//     fn visit_switch(&mut self, switch: &Switch) {
//         self.0.visit_expression(&switch.expression);
//         for clause in switch.cases.iter() {
//             self.0.visit_literal(&clause.literal);
//             self.0.visit_block(&clause.block);
//         }
//         if let Some(block) = &switch.default {
//             self.0.visit_block(block);
//         }
//     }

//     fn visit_object(&mut self, object: &Object) {
//         self.0.visit_code(&object.code);
//         if let Some(inner) = &object.inner_object {
//             self.0.visit_object(inner)
//         }
//     }

//     fn visit_for_loop(&mut self, for_loop: &ForLoop) {
//         self.0.visit_block(&for_loop.initializer);
//         self.0.visit_expression(&for_loop.condition);
//         self.0.visit_block(&for_loop.finalizer);
//         self.0.visit_block(&for_loop.body);
//     }

//     fn visit_variable_declaration(&mut self, variable_definition: &VariableDeclaration) {
//         if let Some(expr) = &variable_definition.expression {
//             self.visit_expression(expr);
//         }
//     }

//     fn visit_function_definition(&mut self, function_definition: &FunctionDefinition) {
//         self.0.visit_block(&function_definition.body);
//     }

//     fn visit_name(&mut self, _: &Name) {}

//     fn visit_function_call(&mut self, call: &FunctionCall) {
//         self.0.visit_name(&call.name);
//         for a in &call.arguments {
//             self.0.visit_expression(a);
//         }
//     }

//     fn visit_if_conditional(&mut self, if_conditional: &IfConditional) {
//         self.0.visit_expression(&if_conditional.condition);
//         self.0.visit_block(&if_conditional.block);
//     }

//     fn visit_literal(&mut self, _: &Literal) {}

//     fn visit_expression(&mut self, expr: &Expression) {
//         match expr {
//             Expression::FunctionCall(fc) => self.0.visit_function_call(fc),
//             Expression::Identifier(_) => (),
//             Expression::Literal(l) => self.0.visit_literal(l),
//         }
//     }

//     fn visit_assignment(&mut self, assignment: &Assignment) {
//         self.0.visit_expression(&assignment.initializer)
//     }

//     fn visit_statement(&mut self, stmt: &Statement) {
//         match stmt {
//             Statement::Object(o) => self.0.visit_object(o),
//             Statement::Code(c) => self.0.visit_code(c),
//             Statement::Block(b) => self.0.visit_block(b),
//             Statement::Expression(e) => self.0.visit_expression(e),
//             Statement::FunctionDefinition(fd) => self.0.visit_function_definition(fd),
//             Statement::VariableDeclaration(vd) => self.0.visit_variable_declaration(vd),
//             Statement::Assignment(a) => self.0.visit_assignment(a),
//             Statement::IfConditional(i) => self.0.visit_if_conditional(i),
//             Statement::Switch(s) => self.0.visit_switch(s),
//             Statement::ForLoop(f) => self.0.visit_for_loop(f),
//             Statement::Continue(_) => (),
//             Statement::Break(_) => (),
//             Statement::Leave(_) => (),
//         }
//     }

//     fn visit_block(&mut self, block: &Block) {
//         for s in block.statements.iter() {
//             self.0.visit_statement(s);
//         }
//     }

//     fn visit_code(&mut self, code: &Code) {
//         self.0.visit_block(&code.block);
//     }
// }
