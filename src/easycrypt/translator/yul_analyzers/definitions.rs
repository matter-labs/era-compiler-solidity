//!
//! Collect all the definitions in YUL program and derive their EasyCrypt types.
//!

use std::collections::HashMap;
use std::iter::repeat;

use crate::easycrypt::syntax::r#type::Type as ECType;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::yul::parser::identifier::Identifier;
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
use crate::yul::path::full_name::FullName;
use crate::yul::path::tracker::symbol_tracker::SymbolTracker;
use crate::yul::path::tracker::PathTracker;
use crate::yul::visitor::Visitor as YulVisitor;

/// Collect all definitions in the YUL code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definitions {
    /// Tracks current location and allows binding the identifiers in the current scope.
    pub tracker: SymbolTracker<DefinitionInfo>,
    /// Collects all definitions in YUL code.
    pub all_symbols: HashMap<FullName, DefinitionInfo>,
}

impl Definitions {
    pub fn new() -> Self {
        Self {
            tracker: SymbolTracker::new(),
            all_symbols: HashMap::new(),
        }
    }

    fn add_var(&mut self, binding: &Identifier) {
        let name = &binding.inner;
        let path = self.tracker.here().clone();
        let full_name = FullName {
            name: name.clone(),
            path,
        };
        let r#type = ECType::Unknown.clone();
        let definition = DefinitionInfo {
            kind: Kind::Variable,
            r#type,
            full_name: full_name.clone(),
            predefined: false,
        };
        self.all_symbols.insert(full_name, definition.clone());
        self.tracker.add(name, &definition)
    }
}

impl YulVisitor for Definitions {
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
        self.visit_code(&object.code);

        if let Some(inner_object) = &object.inner_object {
            self.tracker.enter_object(&inner_object.identifier);
            self.visit_object(inner_object);
            self.tracker.leave();
        }
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

    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) {
        for binding in &variable_declaration.bindings {
            self.add_var(binding);
        }
    }

    fn visit_function_definition(&mut self, function_definition: &FunctionDefinition) {
        let FunctionDefinition {
            identifier,
            arguments,
            result,
            body,
            ..
        } = function_definition;
        for argument in arguments {
            self.add_var(argument)
        }

        let name = &identifier;
        self.tracker.enter_function(name);

        let path = self.tracker.here().clone();
        let full_name = FullName {
            name: name.to_string(),
            path,
        };
        let r#type = {
            let arg_type = ECType::type_of_vec(
                &repeat(ECType::Unknown)
                    .take(arguments.len())
                    .collect::<Vec<_>>(),
            );
            let ret_type = ECType::Unknown.clone();
            ECType::Arrow(Box::new(arg_type), Box::new(ret_type))
        };
        let definition = DefinitionInfo {
            kind: Kind::Procedure,
            r#type,
            full_name: full_name.clone(),
            predefined: false,
        };
        self.all_symbols.insert(full_name, definition.clone());

        self.tracker.add(name, &definition);

        for return_value in result {
            self.add_var(return_value)
        }

        self.visit_block(body);
        self.tracker.leave();
    }

    fn visit_name(&mut self, _name: &Name) {
        unreachable!()
    }

    fn visit_function_call(&mut self, _call: &FunctionCall) {
        unreachable!()
    }

    fn visit_if_conditional(&mut self, if_conditional: &IfConditional) {
        self.tracker.enter_if_then();
        self.visit_block(&if_conditional.block);
        self.tracker.leave();
    }

    fn visit_literal(&mut self, _lit: &Literal) {
        unreachable!()
    }

    fn visit_expression(&mut self, _expr: &Expression) {
        unreachable!()
    }

    fn visit_assignment(&mut self, _assignment: &Assignment) {
        unreachable!()
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Object(object) => self.visit_object(object),
            Statement::Code(code) => self.visit_code(code),
            Statement::Block(block) => self.visit_block(block),
            Statement::Expression(_) => (),
            Statement::FunctionDefinition(fd) => self.visit_function_definition(fd),
            Statement::VariableDeclaration(vd) => self.visit_variable_declaration(vd),
            Statement::Assignment(_) => (),
            Statement::IfConditional(if_conditional) => self.visit_if_conditional(if_conditional),
            Statement::Switch(switch) => self.visit_switch(switch),
            Statement::ForLoop(for_loop) => self.visit_for_loop(for_loop),
            Statement::Continue(_) | Statement::Break(_) | Statement::Leave(_) => (),
        }
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
