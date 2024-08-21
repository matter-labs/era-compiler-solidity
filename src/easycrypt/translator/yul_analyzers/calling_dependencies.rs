//!
//! Analyzes dependencies between functions so that they can be sorted topologically.
//!

use topological_sort::TopologicalSort;

use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::yul::parser::statement::assignment::Assignment;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::code::Code;
use crate::yul::parser::statement::expression::function_call::name::Name::UserDefined;
use crate::yul::parser::statement::expression::function_call::FunctionCall;
use crate::yul::parser::statement::expression::Expression;
use crate::yul::parser::statement::for_loop::ForLoop;
use crate::yul::parser::statement::function_definition::FunctionDefinition;
use crate::yul::parser::statement::if_conditional::IfConditional;
use crate::yul::parser::statement::object::Object;
use crate::yul::parser::statement::switch::Switch;
use crate::yul::parser::statement::variable_declaration::VariableDeclaration;
use crate::yul::parser::statement::Statement;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::tracker::symbol_tracker::SymbolTracker;
use crate::yul::path::tracker::PathTracker as _;
use crate::yul::visitor::IMPLICIT_CODE_FUNCTION_NAME;
use crate::YulVisitor;

///
/// Collect all definitions in the YUL code.
///
#[derive(Clone, Debug)]
pub struct CallingDependencies<'a> {
    ///
    /// Tracks current location and allows binding the identifiers in the current scope.
    ///
    pub tracker: SymbolTracker<DefinitionInfo>,
    pub topological_sort: TopologicalSort<FullName>,
    pub functions_stack: Vec<FullName>,
    pub symbols: &'a SymbolTable<DefinitionInfo>,
}

impl<'a> CallingDependencies<'a> {
    pub fn new(symbols: &'a SymbolTable<DefinitionInfo>) -> Self {
        Self {
            tracker: SymbolTracker::new(),
            topological_sort: TopologicalSort::new(),
            functions_stack: Vec::new(),
            symbols,
        }
    }

    pub fn current_function(&self) -> Option<FullName> {
        self.functions_stack.last().cloned()
    }
}
impl YulVisitor for CallingDependencies<'_> {
    fn visit_switch(&mut self, switch: &Switch) {
        let Switch { cases, default, .. } = switch;
        for case in cases {
            self.visit_block(&case.block)
        }
        if let Some(block) = default {
            self.visit_block(block)
        }
    }

    fn visit_object(&mut self, object: &Object) {
        self.tracker.enter_object(&object.identifier);
        self.visit_code(&object.code);

        if let Some(inner_object) = &object.inner_object {
            self.visit_object(inner_object);
        }

        self.tracker.leave()
    }

    fn visit_for_loop(&mut self, for_loop: &ForLoop) {
        self.tracker.enter_for1();
        self.visit_block(&for_loop.initializer);
        self.tracker.leave();
        self.tracker.enter_for2();
        self.visit_block(&for_loop.finalizer);
        self.tracker.leave();
        self.tracker.enter_for3();
        self.visit_block(&for_loop.body);
        self.tracker.leave();
    }

    fn visit_function_definition(&mut self, function_definition: &FunctionDefinition) {
        let FunctionDefinition {
            identifier, body, ..
        } = function_definition;

        let dependency = FullName::new(identifier.to_string(), self.tracker.here().clone());
        if let Some(who_depends) = self.current_function() {
            self.topological_sort
                .add_dependency(dependency.clone(), who_depends);
        } else {
            panic!("Attempt to add dependency but there is no parent function");
        }
        self.functions_stack.push(dependency);
        self.tracker.enter_function(identifier);
        self.visit_block(body);
        self.tracker.leave();
        self.functions_stack.pop();
    }

    fn visit_if_conditional(&mut self, if_conditional: &IfConditional) {
        self.tracker.enter_if_then();
        self.visit_block(&if_conditional.block);
        self.tracker.leave();
    }

    fn visit_function_call(&mut self, function_call: &FunctionCall) {
        let FunctionCall {
            name, arguments, ..
        } = function_call;
        for argument in arguments {
            self.visit_expression(argument)
        }

        if let UserDefined(name) = name {
            if let Some(who_depends) = self.current_function() {
                let attempted_dependency = self.symbols.get(&FullName::new(
                    name.to_string(),
                    self.tracker.here().clone(),
                ));
                if let Some(dependency) = &attempted_dependency {
                    self.topological_sort
                        .add_dependency(dependency.yul_name.clone(), who_depends);
                } else {
                    panic!("Can't find dependency {}", name);
                }
            }
        }
    }
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::FunctionCall(fc) => self.visit_function_call(fc),
            Expression::Identifier(_) => (),
            Expression::Literal(_) => (),
        }
    }

    fn visit_assignment(&mut self, assignment: &Assignment) {
        self.visit_expression(&assignment.initializer);
    }

    fn visit_variable_declaration(&mut self, variable_definition: &VariableDeclaration) {
        if let Some(expression) = &variable_definition.expression {
            self.visit_expression(expression)
        }
    }
    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Object(object) => self.visit_object(object),
            Statement::Code(code) => self.visit_code(code),
            Statement::Block(block) => self.visit_block(block),
            Statement::Expression(e) => self.visit_expression(e),
            Statement::FunctionDefinition(fd) => {
                self.visit_function_definition(fd);
            }
            Statement::VariableDeclaration(variable_declaration) => {
                self.visit_variable_declaration(variable_declaration)
            }
            Statement::Assignment(assignment) => self.visit_assignment(assignment),
            Statement::IfConditional(if_conditional) => self.visit_if_conditional(if_conditional),
            Statement::Switch(switch) => self.visit_switch(switch),
            Statement::ForLoop(for_loop) => self.visit_for_loop(for_loop),
            Statement::Continue(_) | Statement::Break(_) | Statement::Leave(_) => (),
        };
    }

    fn visit_block(&mut self, block: &Block) {
        self.tracker.enter_block();

        for statement in &block.statements {
            self.visit_statement(statement)
        }
        self.tracker.leave();
    }

    fn visit_code(&mut self, code: &Code) {
        self.tracker.enter_code();
        self.functions_stack.push(FullName::new(
            IMPLICIT_CODE_FUNCTION_NAME.to_string(),
            self.tracker.here().clone(),
        ));
        self.visit_block(&code.block);
        self.functions_stack.pop();
        self.tracker.leave();
    }
}
