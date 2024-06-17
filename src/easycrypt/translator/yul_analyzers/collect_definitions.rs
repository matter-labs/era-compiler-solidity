//!
//! Collect all the definitions in YUL program and derive their EasyCrypt types.
//!

use std::iter::repeat;

use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::syntax::r#type::Type as ECType;
use crate::easycrypt::translator::definition_info::kind::proc_kind::ProcKind;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::yul::parser::identifier::Identifier;
use crate::yul::parser::statement::block::Block;
use crate::yul::parser::statement::code::Code;
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
use crate::yul::path::Path;
use crate::yul::visitor::implicit_code_function;
use crate::YulVisitor;

/// Collect all definitions in the YUL code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectDefinitions {
    /// Tracks current location and allows binding the identifiers in the current scope.
    pub tracker: SymbolTracker<DefinitionInfo>,
    /// Collects all definitions in YUL code.
    pub all_symbols: SymbolTable<DefinitionInfo>,
}

impl CollectDefinitions {
    pub fn new() -> Self {
        Self {
            tracker: SymbolTracker::new(),
            all_symbols: SymbolTable::new(),
        }
    }

    fn add_var(&mut self, binding: &Identifier, path: &Path) {
        let name = &binding.inner;
        let full_name = FullName {
            name: name.clone(),
            path: path.clone(),
        };
        let r#type = ECType::Unknown.clone();
        let definition = DefinitionInfo {
            kind: Kind::Variable,
            r#type,
            full_name: full_name.clone(),
        };
        self.tracker.add(name, &definition);
        self.all_symbols.insert(&full_name, &definition);
    }

    fn add_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration,
        path: &Path,
    ) {
        for binding in &variable_declaration.bindings {
            self.add_var(binding, path);
        }
    }

    fn add_function_definition(&mut self, function_definition: &FunctionDefinition, path: &Path) {
        let FunctionDefinition {
            identifier,
            arguments,
            result,
            ..
        } = function_definition;
        for argument in arguments {
            self.add_var(argument, path)
        }

        let name = &identifier;

        let full_name = FullName {
            name: name.to_string(),
            path: path.clone(),
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
            kind: Kind::Proc(ProcKind {
                name: ProcName::UserDefined {
                    name: name.to_string(),
                    module: None,
                },
                attributes: Default::default(),
            }),
            r#type,
            full_name: full_name.clone(),
        };
        self.tracker.add(name, &definition);
        self.all_symbols.insert(&full_name, &definition);

        for return_value in result {
            self.add_var(return_value, path)
        }
    }
}

impl YulVisitor for CollectDefinitions {
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
            Statement::FunctionDefinition(fd) => {
                self.visit_function_definition(fd);
                self.add_function_definition(fd, &self.tracker.here().clone())
            }
            Statement::VariableDeclaration(vd) => {
                self.add_variable_declaration(vd, &self.tracker.here().clone())
            }
            Statement::Assignment(_) => (),
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
        self.visit_block(&code.block);
        self.add_function_definition(&implicit_code_function(code), &self.tracker.here().clone());
        self.tracker.leave();
    }
}
