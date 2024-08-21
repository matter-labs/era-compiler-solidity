//!
//! Iterate over all statements in YUL syntax tree using depth-first search in
//! post-order traversal.
//!

use super::Visitor;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::code::Code;
use crate::yul::parser::statement::for_loop::ForLoop;
use crate::yul::parser::statement::function_definition::FunctionDefinition;
use crate::yul::parser::statement::if_conditional::IfConditional;
use crate::yul::parser::statement::object::Object;
use crate::yul::parser::statement::switch::Switch;
use crate::yul::parser::statement::Statement;
use crate::yul::path::builder::Builder;
use crate::yul::path::tracker::PathTracker;
use crate::yul::path::Path;

///
/// State of statement visitor.
///
pub trait StatementAction {
    /// Action to perform on each statement.
    fn action(&mut self, statement: &Statement, path: &Path);
}

///
/// Iterate over all statements in YUL syntax tree using depth-first search in
/// post-order traversal.
///
pub struct Statements<A>
where
    A: StatementAction,
{
    pub action: A,
    tracker: Builder,
}

impl<A> Statements<A>
where
    A: StatementAction,
{
    ///
    /// Returns a new instance of [`Statements`].
    ///
    pub fn new(action: A, path: Path) -> Self {
        Self {
            action,
            tracker: Builder::new(path),
        }
    }
}

impl<A> Visitor for Statements<A>
where
    A: StatementAction,
{
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
        self.tracker.enter_function(identifier);
        self.visit_block(body);
        self.tracker.leave();
    }

    fn visit_if_conditional(&mut self, if_conditional: &IfConditional) {
        self.tracker.enter_if_then();
        self.visit_block(&if_conditional.block);
        self.tracker.leave();
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Object(object) => self.visit_object(object),
            Statement::Code(code) => self.visit_code(code),
            Statement::Block(block) => self.visit_block(block),
            Statement::Expression(_) => (),
            Statement::FunctionDefinition(fd) => self.visit_function_definition(fd),
            Statement::VariableDeclaration(_) | Statement::Assignment(_) => (),
            Statement::IfConditional(if_conditional) => self.visit_if_conditional(if_conditional),
            Statement::Switch(switch) => self.visit_switch(switch),
            Statement::ForLoop(for_loop) => self.visit_for_loop(for_loop),
            Statement::Continue(_) | Statement::Break(_) | Statement::Leave(_) => (),
        };
        self.action.action(stmt, self.tracker.here())
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
        self.visit_block(&code.block);
        self.tracker.leave();
    }
}
